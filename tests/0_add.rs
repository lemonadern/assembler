// 0: Add
// add rd,rs,rt
// type R

mod common;

use std::collections::HashMap;

use assembler::instruction::parse_instruction;

use crate::common::convert_to_string_vec;

#[test]
fn happy() {
    let input = convert_to_string_vec(vec!["add", "$zero", "$zero", "$zero"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());

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
