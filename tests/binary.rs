use assembler::binary::binary_string;

#[test]
fn zero() {
    assert_eq!(binary_string(0_u16, 16), "0000000000000000");
    assert_eq!(binary_string(0_i16 as u16, 16), "0000000000000000");
}

#[test]
fn twos_complements() {
    assert_eq!(binary_string(65535_u16, 16), "1111111111111111");
    assert_eq!(binary_string(-1_i16 as u16, 16), "1111111111111111");
}

#[test]
fn opcodes() {
    // 6-bit value
    assert_eq!(binary_string(0_u16, 6), "000000");
    assert_eq!(binary_string(8_u16, 6), "001000");
    assert_eq!(binary_string(63_u16, 6), "111111");
}

#[test]
fn registers() {
    // 0 to 31 (5-bit)
    assert_eq!(binary_string(0_u16, 5), "00000");
    assert_eq!(binary_string(31_u16, 5), "11111");
}

#[test]
fn addrs() {
    assert_eq!(binary_string(0_u16, 26), "00000000000000000000000000");
    assert_eq!(
        binary_string(67108863_u64, 26),
        "11111111111111111111111111"
    );
}
