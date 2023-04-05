use std::mem::size_of;
use crate::processing::processor::MemoryManagers;
use super::Instruction;

pub struct InvertInstruction {
    address: usize
}

pub const INVERT_INSTRUCTION_CODE: u16 = 1;

/// Inverts the given boolean address from 0x00 to 0xFF
impl InvertInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, to_flip: usize, dest: usize) -> Self {
        let mut instruction_memory = vec![];
        instruction_memory.extend(INVERT_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(to_flip.to_le_bytes());
        instruction_memory.extend(dest.to_le_bytes());

        let address = memory_managers.program_memory.append(instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 { INVERT_INSTRUCTION_CODE }

    pub fn get_size() -> usize {
        size_of::<usize>() * 2 // To flip, dest
    }

    pub fn get_debug(data: &[u8]) -> String {
        format!("INVERT [{}] dest [{}]",
                usize::from_le_bytes((&data[0..size_of::<usize>()]).try_into().unwrap()),
                usize::from_le_bytes((&data[size_of::<usize>()..size_of::<usize>() * 2]).try_into().unwrap()),
        )
    }
}

impl Instruction for InvertInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}