pub fn binary_string<T: Into<u64>>(value: T, bit: usize) -> String {
    let value_u64 = value.into();
    format!("{:0width$b}", value_u64, width = bit)
}
