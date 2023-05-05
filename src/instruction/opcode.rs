use crate::binary::binary_string;

pub struct Opcode {
    id: usize,
    #[allow(dead_code)]
    name: String,
}

impl Opcode {
    pub fn new(id: usize, name: &str) -> Self {
        Opcode {
            id,
            name: name.to_string(),
        }
    }

    pub fn to_binary_string(&self) -> String {
        // This processor has 6-bit opcodes.
        binary_string(self.id as u64, 6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_to_binary() {
        let o = Opcode::new(0, "add");
        assert_eq!(o.to_binary_string(), "000000");

        let o = Opcode::new(7, "j");
        assert_eq!(o.to_binary_string(), "000111");
    }
}
