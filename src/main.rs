mod generation;
use clap::Parser;
use std::io::{prelude::*, BufReader};
use std::fs::File;
use dirs::home_dir;
use std::process;
use crate::generation::*;

/// A program to automatically generate a markdown file from a source code file
/// containing embedded markdown notes on the function of the code
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///Name of source code file
    file: String,

    ///Creates a copy of the source code with a .tidy extension which contains none of the notegen symbols
    #[clap(short, long)]
    tidy_mode: bool,

    ///Puts a "#LanguageName" tag after the first regular markdown line (which will usually be a header) for automatic organization in Obsidian
    #[clap(short, long)]
    generate_tags: bool,

}

struct Config {
    input_file: String,
    ouput_dir: String,
    tidy_mode: bool,
    generate_tags: bool,
}

/**
 * grabs config from args and config file and returns a Config struct
 */
fn get_config_settings() -> Config {
    let args = Args::parse();

    let home = home_dir();
    let config_path = home.unwrap();
    let mut config_str = config_path.to_str().unwrap().to_string();
    config_str.push_str("/.config/notegen/config.txt");
    
    //open a file at ~/.config/notegen/config.txt and read it line by line into the Config struct
    let config_result = File::open(config_str);
    let mut config_vec: Vec<String> = vec!();
    if let Ok(file) = config_result {
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            config_vec.push(line);
        }
        return Config {
            input_file: args.file,
            ouput_dir: config_vec[0].to_string(),
            tidy_mode: args.tidy_mode,
            generate_tags: config_vec[1].to_string().parse::<bool>().unwrap(),
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
    let reader = BufReader::new(&source_file);

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

    let generated_files = generate(parsed_tokens.unwrap(), &source_tokens, ext.clone(), config.generate_tags);


    //write each element of generated files to the output directory
    //if the file path is "" then place in the current directory
    for file in generated_files {
        let mut file_path = config.ouput_dir.clone();
        if file_path.eq("") {
            file_path = ".".to_string();
        }
        file_path.push_str("/");
        file_path.push_str(&file.0);
        file_path.push_str(".md");
        let new_md_file = File::create(file_path);
        if let Err(e) = &new_md_file {
            println!("\nLooks like that directory doesn't exist: {}", e);
            process::exit(1);
        }
        let mut new_md_file = new_md_file.unwrap();
        new_md_file.write_all(file.1.as_bytes()).unwrap();
    }

    //if tidy mode is enabled, duplicate the source file to a .tidy file and remove all of the lines that contain "//@"
    if config.tidy_mode {
        let mut tidy_file = File::create(config.input_file.clone() + ".orig").unwrap();
        //delete the original file
        std::fs::remove_file(config.input_file.clone()).unwrap();
        //open a new file with the original file name
        let mut source_file = File::create(config.input_file.clone()).unwrap();
        for line in source_tokens {
            if !line.0.contains("//@") {
                source_file.write_all(line.0.as_bytes()).unwrap();
                source_file.write_all("\n".as_bytes()).unwrap();
            }
            tidy_file.write_all(line.0.as_bytes()).unwrap();
            tidy_file.write_all("\n".as_bytes()).unwrap();
        }
    }

    println!("\nDone!");

}
