use std::mem::size_of;
use std::time::Instant;
use crate::processing::instructions::and_instruction_6::AndInstruction;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::equal_instruction_7::EqualInstruction;
use crate::processing::instructions::invert_instruction_1::InvertInstruction;
use crate::processing::instructions::jump_if_not_instruction_2::JumpIfNotInstruction;
use crate::processing::instructions::jump_instruction_3::JumpInstruction;
use crate::processing::instructions::jump_variable_instruction_4::JumpVariableInstruction;
use crate::processing::instructions::or_instruction_8::OrInstruction;
use crate::processing::instructions::print_instruction_5::PrintInstruction;
use crate::processing::processor::MemoryManagers;

pub fn get_u8(pointer: &usize, memory: &[u8]) -> u8 {
    u8::from_le_bytes((&memory[*pointer..(*pointer + 1)]).try_into().unwrap())
}

pub fn get_usize(pointer: &usize, memory: &[u8]) -> usize {
    usize::from_le_bytes((&memory[*pointer..(*pointer + size_of::<usize>())]).try_into().unwrap())
}

pub fn execute(memory_managers: &mut MemoryManagers) -> Result<(), String> {
    let mut pointer: usize = 0;
    let program_length = memory_managers.program_memory.memory.len();

    println!("Executing program");
    let start_time = Instant::now();

    while pointer < program_length {
        let code = &memory_managers.program_memory.memory[pointer..pointer+2];
        pointer += 2;

        match u16::from_le_bytes(code.try_into().unwrap()) {
            0 => CopyInstruction::execute(&mut pointer, memory_managers),
            1 => InvertInstruction::execute(&mut pointer, memory_managers),
            2 => JumpIfNotInstruction::execute(&mut pointer, memory_managers),
            3 => JumpInstruction::execute(&mut pointer, memory_managers),
            4 => JumpVariableInstruction::execute(&mut pointer, memory_managers),
            5 => PrintInstruction::execute(&mut pointer, memory_managers),
            6 => AndInstruction::execute(&mut pointer, memory_managers),
            7 => EqualInstruction::execute(&mut pointer, memory_managers),
            8 => OrInstruction::execute(&mut pointer, memory_managers),
            code => return Err(format!("Unknown code! [{}]", code))
        };

    }

    println!("Execution completed [{:?}]", start_time.elapsed());

    Ok(())
}