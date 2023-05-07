mod binary;
mod cli;
mod instruction;
mod parser;

use std::{
    fs::File,
    io::{Read, Write},
};

use clap::Parser;
use cli::Cli;

use crate::{
    instruction::parse_instruction,
    parser::{parse_asm, remove_comments},
};

fn main() {
    let args = Cli::parse();

    let input_path = args.input_file;
    let base_address = args.base_address; // Default: 0
    let output = &args.output; // Default: "output.txt"

    let mut file = match File::open(&input_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file {}: {}", input_path.display(), error);
            return;
        }
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error reading `{}`: {}", input_path.display(), error);
            return;
        }
    }

    let (instructions, label_map) = parse_asm(&remove_comments(content.as_str()));

    // // println!("{:#?}", &instructions);
    // // println!("{:#?}", &label_map);

    let mut errors = Vec::new();
    let binaries: Vec<String> = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, base_address, &label_map))
        .enumerate()
        .filter_map(|(i, r)| r.map_err(|e| errors.push((i, e))).ok())
        .map(|x| x.encode_to_binary())
        .collect();

    // // println!("{:#?}", errors);
    // // println!("{:#?}", binaries);

    if !errors.is_empty() {
        eprintln!("Error occurred while assembling.");
        println!();
        println!("Found Errors:");
        for (index, error) in errors {
            println!("  At index {:3}, {}", index, error);
        }
        println!();
        return;
    }

    let mut file = match File::create(output) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Failed to open file: {}", error);
            return;
        }
    };

    let content = binaries.join("\n");
    if let Err(error) = write!(file, "{}", content) {
        eprintln!("Failed to write to file: {}", error);
        return;
    }

    if let Err(error) = file.flush() {
        eprintln!("Failed to flush file: {}", error);
        return;
    }

    println!("Assembling completed.")
}
