use std::mem::size_of;
use crate::execution::get_usize;
use crate::processing::processor::MemoryManagers;
use super::Instruction;

pub struct DynamicToCopyInstruction {
    address: usize
}

pub const DYNAMIC_TO_COPY_INSTRUCTION_CODE: u16 = 11;

impl DynamicToCopyInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, direct_from: usize, to_pointer_location: usize, length: usize)
                     -> Self {

        let mut instruction_memory = vec![];
        instruction_memory.extend(DYNAMIC_TO_COPY_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(direct_from.to_le_bytes());
        instruction_memory.extend(to_pointer_location.to_le_bytes());
        instruction_memory.extend(length.to_le_bytes());

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 { DYNAMIC_TO_COPY_INSTRUCTION_CODE }

    pub fn get_size() -> usize {
        size_of::<usize>() * 3 // From, To,  Length
    }

    pub(crate) fn get_debug(data: &[u8]) -> String {
        format!("COPY [{}] (len:{}) dest [{}]",
                get_usize(&0, data),
                get_usize(&(size_of::<usize>() * 2), data),
                get_usize(&size_of::<usize>(), data),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let from = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let dynamic_to = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let len = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();

        let actual_to = get_usize(&dynamic_to, &memory_managers.variable_memory.memory);

        for i in 0..len {
            memory_managers.variable_memory.memory[actual_to + i] =
                memory_managers.variable_memory.memory[from + i];
        }
    }
}

impl Instruction for DynamicToCopyInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}