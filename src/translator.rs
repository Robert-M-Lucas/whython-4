use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::invert_instruction_1::InvertInstruction;
use crate::processing::instructions::jump_if_instruction_2::JumpIfInstruction;
use crate::processing::instructions::jump_instruction_3::JumpInstruction;

pub fn translate(data: &Vec<u8>) {
    println!("<------------------------------>");
    let mut i: usize = 0;
    while i < data.len() {
        let code = &data[i..i+2];
        i += 2;
        let (output, size) = match u16::from_le_bytes(code.try_into().unwrap()) {
            0 => {
                (CopyInstruction::get_debug(&data[i..i+CopyInstruction::get_size()]),
                 CopyInstruction::get_size())
            },
            1 => {
                (InvertInstruction::get_debug(&data[i..i+InvertInstruction::get_size()]),
                 InvertInstruction::get_size())
            },
            2 => {
                (JumpIfInstruction::get_debug(&data[i..i+JumpIfInstruction::get_size()]),
                 JumpIfInstruction::get_size())
            },
            3 => {
                (JumpInstruction::get_debug(&data[i..i+JumpInstruction::get_size()]),
                 JumpInstruction::get_size())
            },
            code => panic!("Debug not implemented for code {}", code),
        };

        println!("{}", output);

        i += size;
    }
    println!("<------------------------------>");
}