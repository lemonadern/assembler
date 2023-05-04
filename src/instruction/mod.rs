mod opcode;
mod register;

use crate::binary::binary_string;
use anyhow::{anyhow, Ok, Result};
use regex::Regex;

use self::{opcode::Opcode, register::Register};

pub fn parse_instruction(input: Vec<String>) -> Result<Box<dyn IntoBinaryFormat>> {
    let opcode = input.get(0).expect("Opcode is not found").as_str();

    match opcode {
        "add" => {
            // 0: Add
            // add rd,rs,rt
            // type R
            let rd: Register = input
                .get(1)
                .ok_or_else(|| anyhow!(operand_missing_message("add", "rd")))?
                .try_into()?;
            let rs: Register = input
                .get(2)
                .ok_or_else(|| anyhow!(operand_missing_message("add", "rs")))?
                .try_into()?;
            let rt: Register = input
                .get(3)
                .ok_or_else(|| anyhow!(operand_missing_message("add", "rt")))?
                .try_into()?;

            Ok(Box::new(RTypeInstruction {
                opcode: Opcode::new(0, opcode),
                rs,
                rt,
                rd,
            }))
        }
        "addi" => {
            // 1: Add Immidiate
            // addi rt,rs,imm
            // type I
            let rt: Register = input
                .get(1)
                .ok_or_else(|| anyhow!(operand_missing_message("addi", "rt")))?
                .try_into()?;
            let rs: Register = input
                .get(2)
                .ok_or_else(|| anyhow!(operand_missing_message("addi", "rs")))?
                .try_into()?;

            let imm: isize = input
                .get(3)
                .ok_or_else(|| anyhow!(operand_missing_message("addi", "imm")))?
                .parse()?;

            Ok(Box::new(ITypeInstruction {
                opcode: Opcode::new(1, opcode),
                rs,
                rt,
                imm,
            }))
        }
        "lw" => {
            // 2: Load Word
            // lw rt addr(rs)
            // type I
            //
            // Consider: addr(rs)
            let rt: Register = input
                .get(1)
                .ok_or_else(|| anyhow!(operand_missing_message("lw", "rt")))?
                .try_into()?;
            let second = input
                .get(1)
                .ok_or_else(|| anyhow!(operand_missing_message("lw", "addr(rs)")))?;

            let (addr, rs) = parse_addr_and_register(second)?;

            Ok(Box::new(ITypeInstruction {
                opcode: Opcode::new(2, opcode),
                rs,
                rt,
                imm: addr,
            }))
        }
        "sw" => {
            // 3: Store Word
            // sw rt addr(rs)
            // type I
            //
            // Consider: addr(rs)
            todo!()
        }
        "beq" => {
            // 4: Branch on Equal
            // beq rs,rt,addr
            // type I
            //
            // Consider: labels
            todo!()
        }
        "j" => {
            // 5: Jump
            // j addr
            // type J
            //
            // Consider: labels
            todo!()
        }
        "jal" => {
            // 6: Jump and Link
            // jal addr
            // type J
            //
            // Consider: labels
            todo!()
        }
        "jr" => {
            // 7: Jump Register
            // jr rs
            // type R
            let rs: Register = input
                .get(1)
                .ok_or_else(|| anyhow!(operand_missing_message("jr", "rs")))?
                .try_into()?;

            Ok(Box::new(RTypeInstruction {
                opcode: Opcode::new(0, opcode),
                rs,
                rt: Register::default(),
                rd: Register::default(),
            }))
        }
        _ => Err(anyhow!(
            "Unsupported instruction encounted: `{}` is not supported.",
            opcode
        )),
    }
}

fn operand_missing_message(operation: &str, operand: &str) -> String {
    format!(
        "Invalid operand for `{}`: `{}` is missing",
        operation, operand
    )
}

fn parse_addr_and_register(input: &str) -> Result<(isize, Register)> {
    let re = Regex::new(r"(-?\d+)\((.+)\)").expect("Failed to compile regular expression");
    if let Some(captures) = re.captures(input) {
        let addr = captures[1].parse()?;
        let register: Register = captures[2].to_owned().try_into()?;
        Ok((addr, register))
    } else {
        Err(anyhow!("Invalid operand format: {}", input))
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
            // `shamt` and `funct` are not used in the current processor.
            // shamt (5bit)
            binary_string(0 as u64, 5),
            // funct (6bit)
            binary_string(0 as u64, 6),
        )
    }
}

// I: immidiate type instruction
struct ITypeInstruction {
    pub opcode: Opcode,
    pub rs: Register,
    pub rt: Register,
    pub imm: isize,
}

impl IntoBinaryFormat for ITypeInstruction {
    fn encode_to_binary(&self) -> String {
        format!(
            "{}{}{}{}",
            self.opcode.to_binary_string(),
            self.rs.to_binary_string(),
            self.rt.to_binary_string(),
            // 16bit imm/addr value
            binary_string(self.imm as u64, 16)
        )
    }
}

// J: jump type instruction
struct JTypeInstruction {
    pub opcode: Opcode,
    pub addr: isize,
}

impl IntoBinaryFormat for JTypeInstruction {
    fn encode_to_binary(&self) -> String {
        format!(
            "{}{}",
            self.opcode.to_binary_string(),
            // 26bit addr value
            binary_string(self.addr as u64, 26)
        )
    }
}
