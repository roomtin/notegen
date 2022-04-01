use regex::Regex;
use std::io::{Error, ErrorKind};
use std::fmt::Write;

const VALID_TOKENS: [&str; 4] = ["@", " ", "{", "}"];

const TAG_STRINGS: [(&str, &str); 4] =
    [("rs", "#Rust"), ("c","#C"), ("java","#Java") , ("go","#Go")];

/**
 * Lexes a source code file and returns a vector of all 
 * the lines containing notegen symbols. Throws an error
 * if the file cannot be read or if there are non notegen
 * symbols in the file.
 */

pub fn lex(mut source_tokens: Vec<(String, usize)>) -> Result<Vec<(String, usize)>, Error> {

    //could use multi line comment support later
    let regex = Regex::new(r"\s*(//@)");
    source_tokens.retain(|line| regex.clone().unwrap().is_match(line.0.as_str()));

    //check to make sure file actually has notegen symbols in it
    if source_tokens.len() == 0 {
        return Err(Error::new(ErrorKind::InvalidData, "No notegen symbols found in file\n"));
    }

    //remove the whitespace from the beginning of each line if it exists
    for mut lines in &mut source_tokens.to_owned() {
        lines.0 = lines.0.trim().to_string();
    }

    match validate_tokens(&source_tokens) {
        Ok(()) =>  return Ok(source_tokens.clone()),
        Err(e) => Err(e),
    }

}

/**
 * Helper for lex. Checks that all the tokens in the source code file are valid.
 * make sure that each string in tokens begins with "//@X" where X is a valid token
 */
fn validate_tokens(tokens: &Vec<(String, usize)>) -> Result<(), Error> {
    let mut error_string = String::new();
    for token in tokens {
        //check if the 3rd character is a valid notegen token
        if !VALID_TOKENS.contains(&&token.0.as_str()[3..4]) {
            write!{&mut error_string, "Invalid notegen Symbol \"{}\" on line {}\n", &token.0, &token.1 + 1}.unwrap();
            return Err(Error::new(ErrorKind::InvalidInput, error_string));
        }
    }
    //check that at least one of the tokens contains a string of "//@@"
    if !tokens.iter().any(|token| token.0.as_str().starts_with("//@@")) {
        write!{&mut error_string, "File contains notegen symbols but no title.\nSpecify markdown file title with the \"//@@\" symbol.\n"}.unwrap();
        return Err(Error::new(ErrorKind::InvalidInput, error_string));
    }
    Ok(()) 
}

/**
 * Parses a vector of tokens and returns a vector of all the parsed tokens.
 * Throws an error if there are any invalid tokens in the source code file,
 * which at this point only happens if there are mismatched brackets.
 */
pub fn parse(tokens: Vec<(String, usize)>) -> Result<Vec<(String, usize)>, Error> {
    let mut bracket_num: i16 = 0;
    let mut error_string = String::new();

    for token in &tokens {
        if token.0.as_str()[3..4].eq("{") {
            bracket_num += 1;
        } else if token.0.as_str()[3..4].eq("}") {
            bracket_num -= 1;
        }
        if bracket_num > 1 {
            write!{&mut error_string, "Missing closing bracket for bracket on line {}\n", &token.1 + 1}.unwrap();
            return Err(Error::new(ErrorKind::InvalidInput, error_string));
        } else if bracket_num < 0 {
            write!{&mut error_string, "Missing opening bracket for bracket on line {}\n", &token.1 + 1}.unwrap();
            return Err(Error::new(ErrorKind::InvalidInput, error_string));
        }
    }
    if bracket_num < 0 {
        write!{&mut error_string, "Missing opening bracket for bracket on line {}\n", &tokens[tokens.len()-1].1 + 1}.unwrap();
        return Err(Error::new(ErrorKind::InvalidInput, error_string));
    } else if bracket_num > 0 {
        write!{&mut error_string, "Missing closing bracket for bracket on line {}\n", &tokens[0].1 + 1}.unwrap();
        return Err(Error::new(ErrorKind::InvalidInput, error_string));
    }

    Ok(tokens)
}

/**
 * Generate a markdown string from the tokens which will
 * be written to a file later in main.
 */
pub fn generate(tokens: Vec<(String, usize)>, source_tokens: &Vec<(String, usize)>, ext: String, gen_tag: bool) -> Vec<(String, String)> {
    let mut markdown_files = vec!();
    let mut code_block_title = String::new();
    let mut start_line: usize = 0;
    let mut printed_tag: bool = false;
    //Generate the markdown file for each defined title
    for token in &tokens {
    //find the title of the markdown file
        if token.0.as_str().starts_with("//@@") {
            //4 because of the length of the notegen symbol
            let title_string = token.0.clone()[4..].trim_start().to_string();
            markdown_files.push((title_string, "".to_string()));
        }
        else if token.0.as_str().starts_with("//@ ") {
            //4 because of the length of the notegen symbol
            let mut markdown_string = token.0.clone()[4..].trim_start().to_string();
            let len = markdown_files.len();
            //if generate tags is on and and you haven't written a tag yet, write one
            if gen_tag & !printed_tag{
                //find the tuple in TAG_STRINGS that matches the extension
                let tag_string = TAG_STRINGS.iter().find(|tag| tag.0 == ext).unwrap();
                markdown_string.push_str(&format!("\n{}\n", tag_string.1));
                code_block_title = tag_string.1.clone().to_string();
                printed_tag = true;

            }
            //add the markdown string to the last markdown file
            markdown_files[len -1].1.push_str(&markdown_string);
            //add a new line to keep markdown correct
            markdown_files[len -1].1.push_str(&"\n");
        }

        else if token.0.as_str().starts_with("//@{") {
            start_line = token.1;
        }

        else if token.0.as_str().starts_with("//@}") {
            let end_line = token.1;
            //generate the markdown file
            let code_snippet = get_code_snippet(start_line, end_line, source_tokens);
            let len = markdown_files.len();
            let mut markdown_string = String::new();
            //add the code snippet to the last markdown file
            write!{&mut markdown_string, "\n```{}\n{}```\n", &code_block_title.to_lowercase()[1..], code_snippet}.unwrap();
            //add the markdown string to the last markdown file
            markdown_files[len -1].1.push_str(&markdown_string);
        }
        else {
            println!("\nThis shouldn't happen. Generator encountered unknown token: {}, {}", token.0, token.1);
        }
    }
    markdown_files
}

/**
 * helper for generator. returns the chunk of source code requested.
 */
fn get_code_snippet(start_line: usize, end_line: usize, source_tokens: &Vec<(String, usize)>) -> String {
    let mut code_snippet = String::new();
    for i in 0..source_tokens.len() {
        if source_tokens[i].1 > start_line && source_tokens[i].1 < end_line {
            write!{&mut code_snippet, "{}\n", source_tokens[i].0}.unwrap();
        }
    }
    code_snippet
}