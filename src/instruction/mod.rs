mod opcode;
mod register;

use crate::binary::binary_string;
use anyhow::{anyhow, Ok, Result};

use self::{opcode::Opcode, register::Register};

fn parse_instruction(input: Vec<String>) -> Result<Box<dyn IntoBinaryFormat>> {
    let opcode = input.get(0).expect("Opcode is not found").as_str();

    match opcode {
        "add" => {
            let rd: Register = input
                .get(1)
                .ok_or_else(|| anyhow!("Invalid operand for `add`: `rd` is missing"))?
                .try_into()?;
            let rs: Register = input
                .get(2)
                .ok_or_else(|| anyhow!("Invalid operand for `add`: `rs` is missing"))?
                .try_into()?;
            let rt: Register = input
                .get(3)
                .ok_or_else(|| anyhow!("Invalid operand for `add`: `rt` is missing"))?
                .try_into()?;

            Ok(Box::new(RTypeInstruction {
                opcode: Opcode::new(0, "add"),
                rs,
                rt,
                rd,
            }))
        }
        // TODO: implement instructions
        _ => Err(anyhow!(
            "Unsupported instruction encounted: `{}` is not supported.",
            opcode
        )),
    }
}

trait IntoBinaryFormat {
    fn encode_to_binary(&self) -> String;
}

// R: register type instruction
struct RTypeInstruction {
    pub opcode: Opcode,
    pub rs: Register,
    pub rt: Register,
    pub rd: Register,
}

impl IntoBinaryFormat for RTypeInstruction {
    fn encode_to_binary(&self) -> String {
        format!(
            "{}{}{}{}{}{}",
            self.opcode.to_binary_string(),
            self.rs.to_binary_string(),
            self.rt.to_binary_string(),
            self.rd.to_binary_string(),
            // shamt (5bit)
            binary_string(0, 5),
            // funct (6bit)
            binary_string(0, 6),
        )
    }
}

// I: immidiate type instruction
struct ITypeInstruction {
    pub opcode: Opcode,
    pub rs: Register,
    pub rt: Register,
    pub imm: usize,
}

impl IntoBinaryFormat for ITypeInstruction {
    fn encode_to_binary(&self) -> String {
        format!(
            "{}{}{}{}",
            self.opcode.to_binary_string(),
            self.rs.to_binary_string(),
            self.rt.to_binary_string(),
            // 16bit imm/addr value
            binary_string(self.imm, 16)
        )
    }
}

// J: jump type instruction
struct JTypeInstruction {
    pub opcode: Opcode,
    pub addr: usize,
}

impl IntoBinaryFormat for JTypeInstruction {
    fn encode_to_binary(&self) -> String {
        format!(
            "{}{}",
            self.opcode.to_binary_string(),
            // 26bit addr value
            binary_string(self.addr, 26)
        )
    }
}
