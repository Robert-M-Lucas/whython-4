use std::mem::size_of;
use crate::util::get_usize;
use crate::processing::processor::MemoryManagers;
use crate::processing::types::boolean::{BOOLEAN_FALSE, BOOLEAN_TRUE};
use super::Instruction;

pub struct NotEqualInstruction {
    address: usize
}

pub const NOT_EQUAL_INSTRUCTION_CODE: u16 = 14;

/// Applies not equal to LHS and RHS
impl NotEqualInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, lhs: usize, rhs: usize, len: usize, dest: usize) -> Self {
        let mut instruction_memory = vec![];
        instruction_memory.extend(NOT_EQUAL_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(lhs.to_le_bytes());
        instruction_memory.extend(rhs.to_le_bytes());
        instruction_memory.extend(len.to_le_bytes());
        instruction_memory.extend(dest.to_le_bytes());

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 { NOT_EQUAL_INSTRUCTION_CODE }

    pub fn get_size() -> usize {
        size_of::<usize>() * 4 // LHS, RHS, len, dest
    }

    pub fn get_debug(data: &[u8]) -> String {
        format!("NOT EQUAL [{}], [{}] (len:{}) dest [{}]",
                get_usize(&0, data),
                get_usize(&size_of::<usize>(), data),
                get_usize(&(size_of::<usize>() * 2), data),
                get_usize(&(size_of::<usize>() * 3), data),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let lhs = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let rhs = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let len = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let dest = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();


        for i in 0..len {
            if memory_managers.variable_memory.memory[lhs + i] != memory_managers.variable_memory.memory[rhs + i] {
                memory_managers.variable_memory.memory[dest] = BOOLEAN_TRUE;
                return;
            }
        }
        memory_managers.variable_memory.memory[dest] = BOOLEAN_FALSE;
    }
}

impl Instruction for NotEqualInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}