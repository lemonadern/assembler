// 1: Add Immediate
// addi rt,rs,imm
// type I

use std::collections::HashMap;

use assembler::instruction::parse_instruction;

use crate::common::convert_to_string_vec;

mod common;

#[test]
fn happy() {
    let input = convert_to_string_vec(vec!["addi", "$zero", "$zero", "4"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());

    let actual = result.unwrap().encode_to_binary();

    assert_eq!(actual.len(), 32);
    assert_eq!(actual, "00000100000000000000000000000100");
}

#[test]
fn fail_on_excessive_operands() {
    let input = convert_to_string_vec(vec!["addi", "$zero", "$zero", "1", "2"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_insufficient_operands() {
    let input = convert_to_string_vec(vec!["addi", "$zero", "$zero"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_immediate_value_instead_of_register() {
    let input = convert_to_string_vec(vec!["addi", "$zero", "1", "$ra"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_register_instead_of_immediate_value() {
    let input = convert_to_string_vec(vec!["addi", "$zero", "$ra", "$ra"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_invalid_register() {
    let input = convert_to_string_vec(vec!["addi", "$hello", "$ra", "$fp"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}
