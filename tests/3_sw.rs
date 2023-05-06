// 3: Store Word
// sw rt addr(rs)
// type I
//
// Consider: addr(rs)

use std::collections::HashMap;

use assembler::instruction::parse_instruction;

use crate::common::convert_to_string_vec;

mod common;

#[test]
fn happy() {
    let input = convert_to_string_vec(vec!["sw", "$zero", "4($fp)"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());

    let actual = result.unwrap().encode_to_binary();

    assert_eq!(actual.len(), 32);
    assert_eq!(actual, "00001111110000000000000000000100");
}

#[test]
fn fail_on_excessive_operands() {
    let input = convert_to_string_vec(vec!["sw", "$zero", "4($fp)", "$ra"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_insufficient_operands() {
    let input = convert_to_string_vec(vec!["sw", "$zero", "4($fp"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_invalid_register() {
    let input = convert_to_string_vec(vec!["sw", "$hello", "4($fp)"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_offset_out_of_range_positive() {
    let input = convert_to_string_vec(vec!["sw", "$zero", "65536($fp)"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_offset_out_of_range_negative() {
    let input = convert_to_string_vec(vec!["sw", "$zero", "-32769($fp)"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_mismatched_parentheses() {
    let input = convert_to_string_vec(vec!["sw", "$zero", "4($fp"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_misplaced_parentheses() {
    let input = convert_to_string_vec(vec!["sw", "$zero", "4$(fp)"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}

#[test]
fn fail_on_unexpected_whitespace() {
    let input = convert_to_string_vec(vec!["sw", "$zero", "4 ($fp)"]);
    let result = parse_instruction(&input, 0, 0, &HashMap::new());
    assert!(result.is_err())
}
