use std::mem::size_of;
use num_format::Locale::fr;
use crate::execution::get_usize;
use crate::processing::processor::MemoryManagers;
use super::Instruction;

pub struct CopyInstruction {
    address: usize
}

pub const COPY_INSTRUCTION_CODE: u16 = 0;

impl CopyInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, from: usize, to: usize, length: usize)
        -> Self {

        let mut instruction_memory = vec![];
        instruction_memory.extend(COPY_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(from.to_le_bytes());
        instruction_memory.extend(to.to_le_bytes());
        instruction_memory.extend(length.to_le_bytes());

        let address = memory_managers.program_memory.append(instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 { COPY_INSTRUCTION_CODE }

    pub fn get_size() -> usize {
        size_of::<usize>() * 3 // From, To,  Length
    }

    pub(crate) fn get_debug(data: &[u8]) -> String {
        format!("COPY [{}] (len:{}) dest [{}]",
                usize::from_le_bytes((&data[0..size_of::<usize>()])
                    .try_into().unwrap()),
                usize::from_le_bytes((&data[size_of::<usize>() * 2..])
                    .try_into().unwrap()),
                usize::from_le_bytes((&data[size_of::<usize>()..size_of::<usize>() * 2])
                    .try_into().unwrap()),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let from = get_usize(pointer, memory_managers);
        *pointer += size_of::<usize>();
        let to = get_usize(pointer, memory_managers);
        *pointer += size_of::<usize>();
        let len = get_usize(pointer, memory_managers);
        *pointer += size_of::<usize>();

        for i in 0..len {
            memory_managers.variable_memory.memory[to + i] =
                memory_managers.variable_memory.memory[from + i];
        }
    }
}

impl Instruction for CopyInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}