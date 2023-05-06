// Tests for learning

#[test]
fn twos_complements() {
    let i = 0_i16;
    let u = 0_u16;
    assert_eq!(i as u16, u);

    let i = -1_i16;
    let u = 65535_u16;
    assert_eq!(i as u16, u);

    let i = 32767_i16;
    let u = 32767_u16;
    assert_eq!(i as u16, u);

    let i = -32768_i16;
    let u = 32768_u16;
    assert_eq!(i as u16, u);
}
