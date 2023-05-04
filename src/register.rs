use anyhow::{anyhow, Result};

use crate::binary::to_padded_binary_string;

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

    pub fn to_binary_string(&self, width: usize) -> String {
        to_padded_binary_string(self.id, width)
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
