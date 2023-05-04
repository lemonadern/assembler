mod binary;
mod instruction;
mod parser;

use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{
    instruction::parse_instruction,
    parser::{parse_asm, remove_comments},
};

fn main() -> anyhow::Result<()> {
    let filename = "asm.txt";
    let mut file =
        File::open(&filename).expect(format!("File `{}` is not Found.", &filename).as_str());
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("something went wrong reading the file");
    // println!("File: {}", content);

    let (instructions, label_map) = parse_asm(&remove_comments(content.as_str()));

    // println!("{:#?}", &instructions);
    println!("{:#?}", &label_map);

    let base_address = 0;

    let binaries: Vec<String> = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, base_address, &label_map).unwrap())
        .map(|x| x.encode_to_binary())
        .collect();

    println!("{:#?}", binaries);

    let mut file = File::create("output.txt").expect("Failed to open file.");
    write!(file, "{}", binaries.join("\n"))?;
    file.flush()?;
    Ok(())
}
