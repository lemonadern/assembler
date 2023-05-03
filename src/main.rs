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
    println!("Hello, world!");
}
