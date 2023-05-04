mod binary;
mod instruction;
mod parser;

use crate::{
    instruction::parse_instruction,
    parser::{parse_asm, remove_comments},
};

fn main() {
    let input = "addi    $sp,    $sp,    -4\n main : sw      $ra,    0($sp)";
    let (instructions, label_map) = parse_asm(&remove_comments(input));

    println!("{:?}", &instructions);
    println!("{:?}", &label_map);

    let base_address = 0;

    let binaries: Vec<String> = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, base_address, &label_map).unwrap())
        .map(|x| x.encode_to_binary())
        .collect();

    println!("{:?}", binaries);
}
