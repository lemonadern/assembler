mod binary;
mod instruction;
mod parser;

use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use crate::{
    instruction::parse_instruction,
    parser::{parse_asm, remove_comments},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo r <file_path>");
        return;
    }

    let input_path = &args[1];
    let mut file = match File::open(input_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file {}: {}", input_path, error);
            return;
        }
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error reading `{}`: {}", input_path, error);
            return;
        }
    }

    let (instructions, label_map) = parse_asm(&remove_comments(content.as_str()));

    // println!("{:#?}", &instructions);
    // println!("{:#?}", &label_map);

    // TODO: base_address も何らかの形で受け取るようにする
    let base_address = 0;

    let mut errors = Vec::new();
    let binaries: Vec<String> = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, base_address, &label_map))
        .enumerate()
        .filter_map(|(i, r)| r.map_err(|e| errors.push((i, e))).ok())
        .map(|x| x.encode_to_binary())
        .collect();

    // println!("{:#?}", errors);
    // println!("{:#?}", binaries);

    if !errors.is_empty() {
        eprintln!("Error occured while assembling.");
        println!("");
        println!("Found Errors:");
        for (index, error) in errors {
            println!("  At index {:3}, {}", index, error);
        }
        println!("");
        return;
    }

    // TODO: output のファイル名指定もできるようにしたい
    let mut file = match File::create("output.txt") {
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
