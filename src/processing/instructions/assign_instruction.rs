use std::mem::size_of;
use crate::processing::processor::MemoryManagers;
use super::Instruction;

pub struct AssignInstruction {
    address: usize
}

const ASSIGN_INSTRUCTION_CODE: u16 = 0;

impl AssignInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, from: usize, to: usize, length: usize) -> Self {
        let mut instruction_memory = vec![];
        instruction_memory.extend(ASSIGN_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(from.to_le_bytes());
        instruction_memory.extend(to.to_le_bytes());
        instruction_memory.extend(length.to_le_bytes());

        let address = memory_managers.program_memory.append(instruction_memory);

        Self { address }
    }
}

impl Instruction for AssignInstruction {
    fn get_code(&self) -> u16 { ASSIGN_INSTRUCTION_CODE }

    fn get_address(&self) -> usize {
        self.address
    }

    fn get_size(&self) -> usize {
        size_of::<usize>() * 3 // From, To,  Length
    }
}