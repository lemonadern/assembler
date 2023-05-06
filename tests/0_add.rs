// 0: Add
// add rd,rs,rt
// type R

use std::collections::HashMap;

use assembler::{instruction::parse_instruction, parser::parse_asm};

#[test]
fn happy() {
    let (instructions, label_map) = parse_asm("add $zero, $zero, $zero");
    let result = parse_instruction(instructions.get(0).unwrap(), 0, 0, &label_map);

    let actual = result.unwrap().encode_to_binary();

    assert_eq!(actual.len(), 32);
    assert_eq!(actual, "00000000000000000000000000000000");
}

#[test]
fn fail_on_excessive_operands() {
    let input = convert_to_string_vec(vec!["add", "$zero", "$zero", "$zero", "$zero"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_insufficient_operands() {
    let input = convert_to_string_vec(vec!["add", "$zero", "$zero"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_immediate_value_instead_of_register() {
    let input = convert_to_string_vec(vec!["add", "$zero", "$zero", "0"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_invalid_register() {
    let input = convert_to_string_vec(vec!["add", "$zero", "$zero", "$hello"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

fn convert_to_string_vec(str_vec: Vec<&str>) -> Vec<String> {
    str_vec.iter().map(|s| s.to_string()).collect()
}
