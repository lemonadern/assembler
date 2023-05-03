use std::collections::HashMap;

fn remove_comments(input: &str) -> String {
    let mut output = String::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_comments() {
        let input =
            "Hello, world!\nRust\n# This is a comment\nAnother line.\nHi, #this is comment, too.\n";
        let expected = "Hello, world!\nRust\nAnother line.\nHi, \n";
        let actual = remove_comments(input);
        assert_eq!(actual, expected);
    }
}

fn main() {
    let input = "addi    $sp,    $sp,    -4\n main : sw      $ra,    0($sp)";
    let converted = parse_asm(input);
    println!("{:?}", converted);
}

type LabelMap = HashMap<String, usize>;
type Operation = Vec<String>;

fn parse_asm(input: &str) -> (Vec<Operation>, LabelMap) {
    let mut label_map = HashMap::new();
    let operations = input
        .lines()
        // Trim whitespaces and remove empty lines
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
            if let Some((label, operation)) = text.split_once(":") {
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
