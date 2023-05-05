use std::{collections::HashMap, vec};

use assembler::parser::{parse_asm, remove_comments};

#[test]
fn test_remove_comments() {
    let input = "sw $ra, 4($sp) # Save $ra \nsw $a0, 0($sp)  # Save $a0 \n";
    assert_eq!(
        remove_comments(input),
        "sw $ra, 4($sp) \nsw $a0, 0($sp)  \n"
    )
}

#[test]
fn test_parse_asm() {
    let input = "sw $ra, 4($sp) \nsw $a0, 0($sp)  \n";
    let expected_instructions = vec![
        vec!["sw", "$ra", "4($sp)"]
            .iter()
            .map(|x| str::to_string(x))
            .collect(),
        vec!["sw", "$a0", "0($sp)"]
            .iter()
            .map(|x| str::to_string(x))
            .collect(),
    ];
    let map = HashMap::new();

    assert_eq!(parse_asm(input), (expected_instructions, map))
}
