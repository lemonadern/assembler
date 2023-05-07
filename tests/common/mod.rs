pub fn convert_to_string_vec(str_vec: Vec<&str>) -> Vec<String> {
    str_vec.iter().map(|s| s.to_string()).collect()
}
