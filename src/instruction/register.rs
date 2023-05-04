use anyhow::{anyhow, Ok, Result};

use crate::binary::binary_string;

#[derive(Clone, Debug)]
pub struct Register {
    id: usize,
    name: String,
}
impl Register {
    pub fn new(id: usize, name: &str) -> Self {
        Register {
            id,
            name: name.to_string(),
        }
    }

    pub fn to_binary_string(&self) -> String {
        // The registers in this processor are 5 bits.
        binary_string(self.id as u64, 5)
    }
}

// better-way? This is an implementation for `rt` and `rd` when using the `jr` instruction.
// This usecase may not be semantically desirable.
impl Default for Register {
    fn default() -> Self {
        Self {
            id: 0,
            name: "$zero".to_string(),
        }
    }
}

fn parse_register(input: &str) -> Result<Register> {
    if !input.starts_with("$") {
        return Err(anyhow!("Invalid Register format"));
    }

    match input {
        "$0" | "$zero" => Ok(Register::new(0, "zero")),
        "$1" | "$at" => Ok(Register::new(1, "at")),
        "$2" | "$v0" => Ok(Register::new(2, "v0")),
        "$3" | "$v1" => Ok(Register::new(3, "v1")),
        "$4" | "$a0" => Ok(Register::new(4, "a0")),
        "$5" | "$a1" => Ok(Register::new(5, "a1")),
        "$6" | "$a2" => Ok(Register::new(6, "a2")),
        "$7" | "$a3" => Ok(Register::new(7, "a3")),
        "$8" | "$t0" => Ok(Register::new(8, "t0")),
        "$9" | "$t1" => Ok(Register::new(9, "t1")),
        "$10" | "$t2" => Ok(Register::new(10, "t2")),
        "$11" | "$t3" => Ok(Register::new(11, "t3")),
        "$12" | "$t4" => Ok(Register::new(12, "t4")),
        "$13" | "$t5" => Ok(Register::new(13, "t5")),
        "$14" | "$t6" => Ok(Register::new(14, "t6")),
        "$15" | "$t7" => Ok(Register::new(15, "t7")),
        "$16" | "$s0" => Ok(Register::new(16, "s0")),
        "$17" | "$s1" => Ok(Register::new(17, "s1")),
        "$18" | "$s2" => Ok(Register::new(18, "s2")),
        "$19" | "$s3" => Ok(Register::new(19, "s3")),
        "$20" | "$s4" => Ok(Register::new(20, "s4")),
        "$21" | "$s5" => Ok(Register::new(21, "s5")),
        "$22" | "$s6" => Ok(Register::new(22, "s6")),
        "$23" | "$s7" => Ok(Register::new(23, "s7")),
        "$24" | "$t8" => Ok(Register::new(24, "t8")),
        "$25" | "$t9" => Ok(Register::new(25, "t9")),
        "$26" | "$k0" => Ok(Register::new(26, "k0")),
        "$27" | "$k1" => Ok(Register::new(27, "k1")),
        "$28" | "$gp" => Ok(Register::new(28, "gp")),
        "$29" | "$sp" => Ok(Register::new(29, "sp")),
        "$30" | "$fp" => Ok(Register::new(30, "fp")),
        "$31" | "$ra" => Ok(Register::new(31, "ra")),
        _ => Err(anyhow!(
            "Unsupported register encounted: `{}` is not supported.",
            input
        )),
    }
}

impl TryFrom<&String> for Register {
    type Error = anyhow::Error;
    fn try_from(value: &String) -> std::result::Result<Self, Self::Error> {
        parse_register(value.as_str())
    }
}

impl TryFrom<String> for Register {
    type Error = anyhow::Error;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        parse_register(value.as_str())
    }
}
