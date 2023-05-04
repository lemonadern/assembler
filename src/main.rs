use anyhow::{anyhow, Ok, Result};

mod binary;
mod instruction;
mod opcode;
mod parser;
mod register;

use crate::binary::binary_string;
use crate::parser::{parse_asm, remove_comments};
use crate::register::Register;

fn main() {
    let input = "addi    $sp,    $sp,    -4\n main : sw      $ra,    0($sp)";
    let (instructions, label_map) = parse_asm(&remove_comments(input));

    println!("{:?}", &instructions);
    println!("{:?}", &label_map);
}

type AssemblyInstruction = Vec<String>;

// WARNING: VERY MESSY CODE
fn evaluate_and_encode(instruction: AssemblyInstruction) -> Result<String> {
    let opcode = instruction.get(0).expect("Opcode is not found").as_str();

    match opcode {
        "add" => {
            let rd: Register = instruction
                .get(1)
                .ok_or_else(|| anyhow!("Invalid operand for `add`: `rd` is missing"))?
                .try_into()?;
            let rs: Register = instruction
                .get(2)
                .ok_or_else(|| anyhow!("Invalid operand for `add`: `rs` is missing"))?
                .try_into()?;
            let rt: Register = instruction
                .get(3)
                .ok_or_else(|| anyhow!("Invalid operand for `add`: `rt` is missing"))?
                .try_into()?;
            // TODO: change to binary
            Ok(format!(
                "add: {}, rs: {}, rt: {}, rd: {}",
                binary_string(0, 6), // `add` instruction number is 0
                rs.to_binary_string(),
                rt.to_binary_string(),
                rd.to_binary_string()
            ))
        }
        // TODO: implement instructions
        _ => Err(anyhow!(
            "Unsupported instruction encounted: `{}` is not supported.",
            opcode
        )),
    }
}
