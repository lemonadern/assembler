pub fn to_padded_binary_string(value: usize, width: usize) -> String {
    format!("{:0width$b}", value, width = width)
}
