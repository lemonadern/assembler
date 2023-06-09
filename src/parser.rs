use std::collections::HashMap;

pub fn remove_comments(input: &str) -> String {
    let mut output = String::new();

    for line in input.lines() {
        if let Some(pos) = line.find('#') {
            output.push_str(&line[..pos]);
            if pos != 0 {
                output.push('\n');
            }
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }

    output
}

pub type LabelMap = HashMap<String, usize>;

pub fn parse_asm(input: &str) -> (Vec<Vec<String>>, LabelMap) {
    let mut label_map = HashMap::new();
    let operations = input
        .lines()
        // Trim whitespace and remove empty lines
        .filter_map(|line| {
            let text = line.trim();
            if text.is_empty() {
                None
            } else {
                Some(text)
            }
        })
        .enumerate()
        .map(|(i, text)| {
            // Read labels and set them in the label_map
            // Convert to an operation iterator
            if let Some((label, operation)) = text.split_once(':') {
                label_map.insert(label.trim().to_owned(), i);
                operation
            } else {
                text
            }
        })
        .map(|s| {
            // Split the operation
            s.split(|c: char| c == ',' || c.is_whitespace())
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_owned())
                .collect()
        })
        .collect();

    (operations, label_map)
}
