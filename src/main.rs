mod binary;
mod instruction;
mod parser;

use crate::parser::{parse_asm, remove_comments};

fn main() {
    let input = "addi    $sp,    $sp,    -4\n main : sw      $ra,    0($sp)";
    let (instructions, label_map) = parse_asm(&remove_comments(input));

    println!("{:?}", &instructions);
    println!("{:?}", &label_map);
}
