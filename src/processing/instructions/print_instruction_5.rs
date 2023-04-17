use std::mem::size_of;
use crate::execution::get_usize;
use crate::processing::processor::MemoryManagers;
use crate::processing::types::Type;
use super::Instruction;

pub struct PrintInstruction {
    address: usize
}

pub const PRINT_INSTRUCTION_CODE: u16 = 5;

impl PrintInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, to_print: &Type)
                     -> Self {

        let mut instruction_memory = vec![];
        instruction_memory.extend(PRINT_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(to_print.get_address().to_le_bytes());
        instruction_memory.extend(to_print.get_size().to_le_bytes());

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 { PRINT_INSTRUCTION_CODE }

    pub fn get_size() -> usize {
        size_of::<usize>() * 2 // Address, LEn
    }

    pub(crate) fn get_debug(data: &[u8]) -> String {
        format!("PRINT [{}] (len:{})",
                get_usize(&0, data),
                get_usize(&size_of::<usize>(), data),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let position = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let len = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        println!("{:X?}", &memory_managers.variable_memory.memory[position..(position + len)]);

    }
}

impl Instruction for PrintInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}