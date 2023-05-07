// 4: Branch on Equal
// beq rs,rt,addr
// type I
//
// `if rs = rt then
//   pc <= pc4 + addr`
// (relative address)
//
// Consider: labels

use assembler::{instruction::parse_instruction, parser::parse_asm};

#[test]
fn positive_pc_relative_addressing() {
    let input = " beq $s1, $s2 NEXT
                        test_operation
                        test_operation
                        NEXT: add $t0,$s1,$s2";

    let (instructions, label_map) = parse_asm(input);

    let mut results = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, 0, &label_map));

    let bin = results.nth(0).unwrap().unwrap().encode_to_binary();

    assert_eq!(bin.len(), 32);
    //               000100 (4: beq)
    //               |    |10001 (17: $s1)
    //               |    ||   |10010 (18: $s2)
    //               |    ||   ||   |0000000000000010 (2: LABEL_address(3) - beq_address(0) - 1)
    //               |    ||   ||   ||              |
    assert_eq!(bin, "00010010001100100000000000000010");
}

#[test]
fn negative_pc_relative_addressing() {
    let input = "BACK: add $t0, $s1, $s2
                  test_operation
                  test_operation
                  test_operation
                  beq $s1, $s2, BACK";

    let (instructions, label_map) = parse_asm(input);

    let mut results = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, 0, &label_map));

    let bin = results.nth(4).unwrap().unwrap().encode_to_binary();

    assert_eq!(bin.len(), 32);
    //               000100 (4: beq)
    //               |    |10001 (17: $s1)
    //               |    ||   |10010 (18: $s2)
    //               |    ||   ||   |1111111111111011 (-5: LABEL_address(0) - beq_address(4) - 1)
    //               |    ||   ||   ||              |
    assert_eq!(bin, "00010010001100101111111111111011");
}

#[test]
fn fail_on_missing_label() {
    let input = "beq $s0, $s1, MISSING";

    let (instructions, label_map) = parse_asm(input);

    let mut results = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, 0, &label_map));

    let result = results.nth(0).unwrap();

    assert!(result.is_err());
}

#[test]
fn fail_on_immediate_value_instead_of_addr() {
    let input = "beq $s0, $s1, 0";

    let (instructions, label_map) = parse_asm(input);

    let mut results = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, 0, &label_map));

    let result = results.nth(0).unwrap();

    assert!(result.is_err());
}

#[test]
fn fail_on_register_instead_of_addr() {
    let input = "beq $s0, $s1, $ra";

    let (instructions, label_map) = parse_asm(input);

    let mut results = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, 0, &label_map));

    let result = results.nth(0).unwrap();

    assert!(result.is_err());
}

#[test]
fn fail_on_excessive_operands() {
    let input = "LABEL: beq $s0, $s1, LABEL, $hi";

    let (instructions, label_map) = parse_asm(input);

    let mut results = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, 0, &label_map));

    let result = results.nth(0).unwrap();

    assert!(result.is_err());
}

#[test]
fn fail_on_insufficient_operands() {
    let input = "LABEL: beq $t0, $s1";

    let (instructions, label_map) = parse_asm(input);

    let mut results = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, 0, &label_map));

    let result = results.nth(0).unwrap();

    assert!(result.is_err());
}
