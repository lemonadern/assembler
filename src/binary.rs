pub fn binary_string<T: Into<u64>>(value: T, bit: usize) -> String {
    format!("{:0width$b}", value.into(), width = bit)
}
