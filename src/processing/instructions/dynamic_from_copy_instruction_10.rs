use std::mem::size_of;
use crate::execution::get_usize;
use crate::processing::processor::MemoryManagers;
use super::Instruction;

pub struct DynamicFromCopyInstruction {
    address: usize
}

pub const DYNAMIC_FROM_COPY_INSTRUCTION_CODE: u16 = 10;

impl DynamicFromCopyInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, from_pointer_location: usize, direct_to: usize, length: usize)
                     -> Self {

        let mut instruction_memory = vec![];
        instruction_memory.extend(DYNAMIC_FROM_COPY_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(from_pointer_location.to_le_bytes());
        instruction_memory.extend(direct_to.to_le_bytes());
        instruction_memory.extend(length.to_le_bytes());

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 { DYNAMIC_FROM_COPY_INSTRUCTION_CODE }

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
        let dynamic_from = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let to = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let len = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();

        let actual_from = get_usize(&dynamic_from, &memory_managers.variable_memory.memory);

        for i in 0..len {
            memory_managers.variable_memory.memory[to + i] =
                memory_managers.variable_memory.memory[actual_from + i];
        }
    }
}

impl Instruction for DynamicFromCopyInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}