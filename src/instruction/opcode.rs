use crate::binary::binary_string;

pub struct Opcode {
    id: usize,
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
        // nktk processor has 6-bit opcodes.
        binary_string(self.id as u64, 6)
    }
}
