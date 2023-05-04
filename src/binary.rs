pub fn binary_string(value: usize, bit: usize) -> String {
    format!("{:0width$b}", value, width = bit)
}
