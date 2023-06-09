mod opcode;
mod register;

use crate::{binary::binary_string, parser::LabelMap};
use anyhow::{anyhow, Ok, Result};
use regex::Regex;

use self::{opcode::Opcode, register::Register};

pub trait IntoBinaryFormat {
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
            "{opcode}{rs}{rt}{rd}{shamt}{funct}",
            opcode = self.opcode.to_binary_string(),
            rs = self.rs.to_binary_string(),
            rt = self.rt.to_binary_string(),
            rd = self.rd.to_binary_string(),
            // `shamt` and `funct` are not used in the current processor.
            // shamt (5bit)
            shamt = binary_string(0_u64, 5),
            // funct (6bit)
            funct = binary_string(0_u64, 6),
        )
    }
}

// I: immediate type instruction
struct ITypeInstruction {
    pub opcode: Opcode,
    pub rs: Register,
    pub rt: Register,
    pub imm: u16,
}

impl IntoBinaryFormat for ITypeInstruction {
    fn encode_to_binary(&self) -> String {
        format!(
            "{opcode}{rs}{rt}{imm_or_addr}",
            opcode = self.opcode.to_binary_string(),
            rs = self.rs.to_binary_string(),
            rt = self.rt.to_binary_string(),
            // 16bit imm/addr value
            imm_or_addr = binary_string(self.imm, 16)
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
            "{opcode}{addr}",
            opcode = self.opcode.to_binary_string(),
            // 26bit addr value
            addr = binary_string(self.addr as u64, 26)
        )
    }
}

pub fn parse_instruction(
    input: &Vec<String>,
    current_index: usize,
    base_address: usize,
    label_map: &LabelMap,
) -> Result<Box<dyn IntoBinaryFormat>> {
    let opcode = input.get(0).expect("Opcode is not found").as_str();

    match opcode {
        "add" => {
            // 0: Add
            // add rd,rs,rt
            // type R

            if input.len() > 4 {
                return Err(anyhow!(too_many_operand_message("add", current_index)));
            }

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
            // 1: Add Immediate
            // addi rt,rs,imm
            // type I

            if input.len() > 4 {
                return Err(anyhow!(too_many_operand_message("addi", current_index)));
            }

            let rt: Register = input
                .get(1)
                .ok_or_else(|| anyhow!(operand_missing_message("addi", "rt")))?
                .try_into()?;
            let rs: Register = input
                .get(2)
                .ok_or_else(|| anyhow!(operand_missing_message("addi", "rs")))?
                .try_into()?;

            let imm: i16 = input
                .get(3)
                .ok_or_else(|| anyhow!(operand_missing_message("addi", "imm")))?
                .parse()?;

            Ok(Box::new(ITypeInstruction {
                opcode: Opcode::new(1, opcode),
                rs,
                rt,
                imm: imm as u16,
            }))
        }
        "lw" => {
            // 2: Load Word
            // lw rt addr(rs)
            // type I
            //
            // Consider: addr(rs)

            if input.len() > 3 {
                return Err(anyhow!(too_many_operand_message("lw", current_index)));
            }

            let rt: Register = input
                .get(1)
                .ok_or_else(|| anyhow!(operand_missing_message("lw", "rt")))?
                .try_into()?;
            let second = input
                .get(2)
                .ok_or_else(|| anyhow!(operand_missing_message("lw", "addr(rs)")))?;

            let (addr, rs) = parse_addr_and_register(second)?;

            Ok(Box::new(ITypeInstruction {
                opcode: Opcode::new(2, opcode),
                rs,
                rt,
                imm: addr as u16,
            }))
        }
        "sw" => {
            // 3: Store Word
            // sw rt addr(rs)
            // type I
            //
            // Consider: addr(rs)

            if input.len() > 3 {
                return Err(anyhow!(too_many_operand_message("sw", current_index)));
            }

            let rt: Register = input
                .get(1)
                .ok_or_else(|| anyhow!(operand_missing_message("sw", "rt")))?
                .try_into()?;
            let second = input
                .get(2)
                .ok_or_else(|| anyhow!(operand_missing_message("sw", "addr(rs)")))?;

            let (addr, rs) = parse_addr_and_register(second)?;

            Ok(Box::new(ITypeInstruction {
                opcode: Opcode::new(3, opcode),
                rs,
                rt,
                imm: addr as u16,
            }))
        }
        "beq" => {
            // 4: Branch on Equal
            // beq rs,rt,addr
            // type I
            //
            // `if rs = rt then
            //   pc <= pc4 + addr`
            // (relative address)
            //
            // Consider: labels

            if input.len() > 4 {
                return Err(anyhow!(too_many_operand_message("beq", current_index)));
            }

            let rs: Register = input
                .get(1)
                .ok_or_else(|| anyhow!(operand_missing_message("beq", "rs")))?
                .try_into()?;
            let rt: Register = input
                .get(2)
                .ok_or_else(|| anyhow!(operand_missing_message("beq", "rt")))?
                .try_into()?;

            // Assuming input only by label
            let label = input
                .get(3)
                .ok_or_else(|| anyhow!(operand_missing_message("beq", "addr")))?;

            let destination_index = label_map.get(label).ok_or_else(|| {
                anyhow!(
                    "The destination label `{}` for the instruction `beq` cannot be found.",
                    &label
                )
            })?;

            let offset = *destination_index as i16 - (current_index + 1) as i16;

            Ok(Box::new(ITypeInstruction {
                opcode: Opcode::new(4, opcode),
                rs,
                rt,
                imm: offset as u16,
            }))
        }
        "j" => {
            // 5: Jump
            // j addr
            // type J
            //
            // `pc <= addr`
            // (absolute address)
            //
            // Consider: labels

            if input.len() > 2 {
                return Err(anyhow!(too_many_operand_message("j", current_index)));
            }

            // Assuming input only by label
            let label = input
                .get(1)
                .ok_or_else(|| anyhow!(operand_missing_message("j", "addr")))?;

            let destination_index = label_map.get(label).ok_or_else(|| {
                anyhow!(
                    "The destination label `{}` for the instruction `j` cannot be found.",
                    &label
                )
            })?;

            let addr = base_address + destination_index;

            Ok(Box::new(JTypeInstruction {
                opcode: Opcode::new(5, opcode),
                addr,
            }))
        }
        "jal" => {
            // 6: Jump and Link
            // jal addr
            // type J
            //
            // `r31 <= pc4
            //  pc <= addr`
            // (absolute address)
            //
            // Consider: labels

            if input.len() > 2 {
                return Err(anyhow!(too_many_operand_message("jal", current_index)));
            }

            // Assuming input only by label
            let label = input
                .get(1)
                .ok_or_else(|| anyhow!(operand_missing_message("jal", "addr")))?;

            let destination_index = label_map.get(label).ok_or_else(|| {
                anyhow!(
                    "The destination label `{}` for the instruction `jal` cannot be found.",
                    &label
                )
            })?;

            let addr = base_address + destination_index;

            Ok(Box::new(JTypeInstruction {
                opcode: Opcode::new(6, opcode),
                addr,
            }))
        }
        "jr" => {
            // 7: Jump Register
            // jr rs
            // type R

            if input.len() > 2 {
                return Err(anyhow!(too_many_operand_message("jr", current_index)));
            }

            let rs: Register = input
                .get(1)
                .ok_or_else(|| anyhow!(operand_missing_message("jr", "rs")))?
                .try_into()?;

            Ok(Box::new(RTypeInstruction {
                opcode: Opcode::new(7, opcode),
                rs,
                rt: Register::default(),
                rd: Register::default(),
            }))
        }
        s => {
            // Although such conditional branching is not desirable,
            // but I have adopted this ad-hoc implementation because there are no plans to extend the assembly language that this assembler interprets.
            if s.starts_with('.') {
                return Err(anyhow!(
                    "Found `{}`, but Directives are not supported in this assembler.",
                    s
                ));
            }

            Err(anyhow!(
                "Unsupported instruction encountered: `{}` is not supported.",
                opcode
            ))
        }
    }
}

fn too_many_operand_message(operation: &str, index: usize) -> String {
    format!("Too many operand for `{}` at {}", operation, index)
}

fn operand_missing_message(operation: &str, operand: &str) -> String {
    format!(
        "Invalid operand for `{}`: `{}` is missing",
        operation, operand
    )
}

fn parse_addr_and_register(input: &str) -> Result<(i16, Register)> {
    let re = Regex::new(r"(-?\d+)\((.+)\)").expect("Failed to compile regular expression");
    if let Some(captures) = re.captures(input) {
        let addr = captures[1].parse()?;
        let register: Register = captures[2].to_owned().try_into()?;
        Ok((addr, register))
    } else {
        Err(anyhow!(
            "Invalid operand format: expecting `addr($register)` but found `{}`",
            input
        ))
    }
}
