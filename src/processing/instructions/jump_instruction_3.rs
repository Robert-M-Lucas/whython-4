use std::mem::size_of;
use crate::execution::get_usize;
use crate::processing::instructions::INSTRUCTION_CODE_LENGTH;
use crate::processing::processor::MemoryManagers;
use crate::processing::types::{Type, TypeSymbol};
use super::Instruction;

pub struct JumpInstruction {
    address: usize
}

pub const JUMP_INSTRUCTION_CODE: u16 = 3;

impl JumpInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, dest: usize) -> Self {
        let mut instruction_memory = vec![];
        instruction_memory.extend(JUMP_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(dest.to_le_bytes());

        let address = memory_managers.program_memory.append(instruction_memory);

        Self { address }
    }

    pub fn set_destination(&self, memory_managers: &mut MemoryManagers, dest: usize) {
        memory_managers.program_memory.overwrite(
            self.address + INSTRUCTION_CODE_LENGTH,
            &dest.to_le_bytes()
        )
    }

    pub fn get_code() -> u16 { JUMP_INSTRUCTION_CODE }

    pub fn get_size() -> usize {
        size_of::<usize>() * 1 // dest
    }

    pub fn get_debug(data: &[u8]) -> String {
        format!("JUMP [{}]",
                usize::from_le_bytes((&data[0..])
                    .try_into().unwrap()),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        *pointer = get_usize(pointer, memory_managers);
    }
}

impl Instruction for JumpInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}