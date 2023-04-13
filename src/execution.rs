use std::mem::size_of;
use std::time::Instant;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::invert_instruction_1::InvertInstruction;
use crate::processing::instructions::jump_if_instruction_2::JumpIfInstruction;
use crate::processing::instructions::jump_instruction_3::JumpInstruction;
use crate::processing::processor::MemoryManagers;

pub fn get_u8(pointer: &usize, memory_managers: &mut MemoryManagers) -> u8 {
    u8::from_le_bytes((&memory_managers.program_memory.memory[*pointer..(*pointer + 1)]).try_into().unwrap())
}

pub fn get_usize(pointer: &usize, memory_managers: &mut MemoryManagers) -> usize {
    usize::from_le_bytes((&memory_managers.program_memory.memory[*pointer..(*pointer + size_of::<usize>())]).try_into().unwrap())
}

pub fn execute(mut memory_managers: MemoryManagers) -> Result<(), String> {
    let mut pointer: usize = 0;
    let program_length = memory_managers.program_memory.memory.len();

    println!("Executing program");
    let start_time = Instant::now();

    while pointer < program_length {
        let code = &memory_managers.program_memory.memory[pointer..pointer+2];
        pointer += 2;

        match u16::from_le_bytes(code.try_into().unwrap()) {
            0 => CopyInstruction::execute(&mut pointer, &mut memory_managers),
            1 => InvertInstruction::execute(&mut pointer, &mut memory_managers),
            2 => JumpIfInstruction::execute(&mut pointer, &mut memory_managers),
            3 => JumpInstruction::execute(&mut pointer, &mut memory_managers),
            code => return Err(format!("Unknown code! [{}]", code))
        };

    }

    println!("Execution completed [{:?}]", start_time.elapsed());

    Ok(())
}