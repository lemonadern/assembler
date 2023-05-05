// Tests for learning

#[test]
fn twos_complements() {
    let i = 0 as i16;
    let u = 0 as u16;
    assert_eq!(i as u16, u);

    let i = -1 as i16;
    let u = 65535 as u16;
    assert_eq!(i as u16, u);

    let i = 32767 as i16;
    let u = 32767 as u16;
    assert_eq!(i as u16, u);

    let i = -32768 as i16;
    let u = 32768 as u16;
    assert_eq!(i as u16, u);
}
