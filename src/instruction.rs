use crate::{binary::binary_string, opcode::Opcode, register::Register};

trait Instruction {
    fn encode_to_binary(&self) -> String;
}

// R: register type instruction
struct RTypeInstruction {
    pub opcode: Opcode,
    pub rs: Register,
    pub rt: Register,
    pub rd: Register,
}

impl Instruction for RTypeInstruction {
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

impl Instruction for ITypeInstruction {
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

impl Instruction for JTypeInstruction {
    fn encode_to_binary(&self) -> String {
        format!(
            "{}{}",
            self.opcode.to_binary_string(),
            // 26bit addr value
            binary_string(self.addr, 26)
        )
    }
}
