mod generation;
use clap::Parser;
use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::path::Path;
use dirs::home_dir;
use std::process;
use crate::generation::*;

//@@ Using Clap to parse arguments
//@ # Clap Struct
//@ Arguments can be parsed using the `clap` crate which allows you
//@ to define your own argument parsing as follows:
//@{
/// A program to automatically generate a markdown file from a source code file
/// containing embedded markdown notes on the function of the code
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///Name of source code file
    file: String,

    ///Removes all of the markdown from the source code file upon completion
    #[clap(short, long)]
    tidy_mode: bool,

    ///Puts a "#LanguageName" tag at the end of the file for automatic organization in Obsidian
    #[clap(short, long)]
    generate_tags: bool,

}
//@}

struct Config {
    input_file: String,
    ouput_dir: String,
    tidy_mode: bool,
    generate_tags: bool,
}

/**
 * grabs config from args and config file and returns a Config struct
 */
//@ Then we can use the `clap` crate's parse function to
//@ parse the arguments as follows:
//@{
fn get_config_settings() -> Config {
    let args = Args::parse();
//@}

    let home = home_dir();
    let config_path = home.unwrap();
    let mut config_str = config_path.to_str().unwrap().to_string();
    config_str.push_str("/.config/notegen/config.txt");
    
    //open a file at ~/.config/notegen/config.txt and read it line by line into the Config struct
    let config_result = File::open(config_str);
    let mut contfig_vec: Vec<String> = vec!();
    if let Ok(file) = config_result {
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            contfig_vec.push(line);
        }
        return Config {
            input_file: args.file,
            ouput_dir: contfig_vec[0].to_string(),
            tidy_mode: args.tidy_mode,
            generate_tags: args.generate_tags,
        };
    }
    else {
        return Config {
            input_file: args.file,
            ouput_dir: "".to_string(),
            tidy_mode: args.tidy_mode,
            generate_tags: args.generate_tags,
        }
    }
}


fn main() {
    let config = get_config_settings();

    let source_result = File::open(&config.input_file);

    if let Err(e) = &source_result {
        println!("Error opening file: {}", e);
        process::exit(1);
    }

    //extract the extension from the file name
    let ext = config.input_file.split(".").last().unwrap().to_string();

    let source_file = source_result.unwrap();
    let mut source_tokens: Vec<(String, usize)> = Vec::new(); 
    let reader = BufReader::new(source_file);

    //push all the lines of the file into a source_tokens vector using their line number as the usize
    for (line_number, line) in reader.lines().enumerate() {
        source_tokens.push((line.unwrap(), line_number));
    }

    let lexed_tokens = lex(source_tokens.clone());
    //lex tokens
    if let Err(e) = &lexed_tokens {
        println!("\n{}", e);
        process::exit(1);
    }
    //parse tokens
    let parsed_tokens = parse(lexed_tokens.unwrap());
    if let Err(e) = &parsed_tokens {
        println!("\n{}", e);
        process::exit(1);
    }

    let generated_files = generate(parsed_tokens.unwrap(), &source_tokens, ext);

    //write each element of generated files to the output directory
    //if the file path is "" then place in the current directory
    for file in generated_files {
        let mut file_path = config.ouput_dir.clone();
        if file_path.eq("") {
            println!("EEZ NOTING");
            file_path = ".".to_string();
        }
        file_path.push_str("/");
        file_path.push_str(&file.0);
        file_path.push_str(".md");
        let mut new_md_file = File::create(file_path).unwrap();
        new_md_file.write_all(file.1.as_bytes()).unwrap();
    }
}