use anyhow::{anyhow, Result};

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
        "$30" | "$fp" => Ok(Register::new(30, input)),
        // TODO: Implement other registers
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
