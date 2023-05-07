// 6: Jump and Link
// jal addr
// type J
//
// `r31 <= pc4
//  pc <= addr`
// (absolute address)
//
// Consider: labels

use assembler::{instruction::parse_instruction, parser::parse_asm};

#[test]
fn jump_positive() {
    let input = " jal NEXT
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
    //               000110 (6: jal)
    //               |    |00000000000000000000000011 (3: base_address(0) + destination_index(3))
    //               |    ||                        |
    assert_eq!(bin, "00011000000000000000000000000011");
}

#[test]
fn jump_negative() {
    let input = " NEXT: add $t0,$s1,$s2
                        test_operation
                        test_operation
                        jal NEXT";

    let (instructions, label_map) = parse_asm(input);

    let mut results = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, 0, &label_map));

    let bin = results.nth(3).unwrap().unwrap().encode_to_binary();

    assert_eq!(bin.len(), 32);
    //               000110: jal)
    //               |    |00000000000000000000000000 (0: base_address(0) + destination_index(0))
    //               |    ||                        |
    assert_eq!(bin, "00011000000000000000000000000000");
}

#[test]
fn addressing() {
    let input = " jal NEXT
                        test_operation
                        test_operation
                        NEXT: add $t0,$s1,$s2";

    let (instructions, label_map) = parse_asm(input);

    let mut results = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, 3, &label_map));

    let bin = results.nth(0).unwrap().unwrap().encode_to_binary();

    assert_eq!(bin.len(), 32);
    //               000110 (6: jal)
    //               |    |00000000000000000000000110 (6: base_address(3) + destination_index(3))
    //               |    ||                        |
    assert_eq!(bin, "00011000000000000000000000000110");
}

#[test]
fn fail_on_missing_label() {
    let input = "jal MISSING";

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
    let input = "jal 0";

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
    let input = "jal $ra";

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
    let input = "LABEL: jal LABEL, $hi";

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
    let input = "LABEL: jal";

    let (instructions, label_map) = parse_asm(input);

    let mut results = instructions
        .iter()
        .enumerate()
        .map(|(i, x)| parse_instruction(x, i, 0, &label_map));

    let result = results.nth(0).unwrap();

    assert!(result.is_err());
}
