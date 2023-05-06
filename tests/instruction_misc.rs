use std::collections::HashMap;

use assembler::instruction::parse_instruction;

#[test]
fn test_fail_to_parse_unsupported_instruction() {
    let r = parse_instruction(&vec!["hi".to_owned()], 0, 0, &HashMap::new());

    match r {
        core::result::Result::Ok(_) => {
            panic!("Expected an error, but got a successful result.")
        }
        Err(err) => {
            let error_message = format!("{}", err);
            assert!(
                error_message.contains("Unsupported instruction"),
                "Error message does not contain expected text: {}",
                error_message
            );
        }
    }
}
