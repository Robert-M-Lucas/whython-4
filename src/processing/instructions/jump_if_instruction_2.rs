use std::mem::size_of;
use crate::processing::instructions::INSTRUCTION_CODE_LENGTH;
use crate::processing::processor::MemoryManagers;
use crate::processing::types::{Type, TypeSymbol};
use super::Instruction;

pub struct JumpIfInstruction {
    address: usize
}

pub const JUMP_IF_INSTRUCTION_CODE: u16 = 2;

impl JumpIfInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, condition_boolean: Type, dest: usize) -> Self {
        if condition_boolean.get_type() != TypeSymbol::Boolean {
            panic!("Jump If instruction can only be created with a boolean condition")
        }

        let mut instruction_memory = vec![];
        instruction_memory.extend(JUMP_IF_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(condition_boolean.get_address().to_le_bytes());
        instruction_memory.extend(dest.to_le_bytes());

        let address = memory_managers.program_memory.append(instruction_memory);

        Self { address }
    }

    pub fn set_destination(&self, memory_managers: &mut MemoryManagers, dest: usize) {
        memory_managers.program_memory.overwrite(
            self.address + INSTRUCTION_CODE_LENGTH + size_of::<usize>(),
            &dest.to_le_bytes()
        )
    }

    pub fn get_code() -> u16 { JUMP_IF_INSTRUCTION_CODE }

    pub fn get_size() -> usize {
        size_of::<usize>() * 2 // Condition, dest
    }

    pub fn get_debug(data: &[u8]) -> String {
        format!("JUMP IF [{}] goto [{}]",
                usize::from_le_bytes((&data[0..size_of::<usize>()])
                    .try_into().unwrap()),
                usize::from_le_bytes((&data[size_of::<usize>()..size_of::<usize>() * 2])
                    .try_into().unwrap()),
        )
    }
}

impl Instruction for JumpIfInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}