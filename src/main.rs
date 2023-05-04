use anyhow::{anyhow, Ok, Result};

mod binary;
mod parser;
mod register;

use crate::binary::to_padded_binary_string;
use crate::parser::{parse_asm, remove_comments};
use crate::register::Register;

fn main() {
    let input = "addi    $sp,    $sp,    -4\n main : sw      $ra,    0($sp)";
    let (operations, label_map) = parse_asm(&remove_comments(input));

    println!("{:?}", &operations);
    println!("{:?}", &label_map);
}

type Operation = Vec<String>;

fn to_binary_string(operation: Operation) -> Result<String> {
    let opcode = operation.get(0).expect("Opcode is not found").as_str();

    match opcode {
        "add" => {
            let rd: Register = operation
                .get(1)
                .ok_or_else(|| anyhow!("Invalid operand for `add`: `rd` is missing"))?
                .try_into()?;
            let rs: Register = operation
                .get(2)
                .ok_or_else(|| anyhow!("Invalid operand for `add`: `rs` is missing"))?
                .try_into()?;
            let rt: Register = operation
                .get(3)
                .ok_or_else(|| anyhow!("Invalid operand for `add`: `rt` is missing"))?
                .try_into()?;
            // TODO: change to binary
            Ok(format!(
                "add: {}, rs: {}, rt: {}, rd: {}",
                to_padded_binary_string(0, 6), // `add` operation number is 0
                rs.to_binary_string(5),
                rt.to_binary_string(5),
                rd.to_binary_string(5)
            ))
        }
        // TODO: implement operations
        _ => Err(anyhow!(
            "Unsupported Operation encounted: `{}` is not supported.",
            opcode
        )),
    }
}
