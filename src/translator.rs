use crate::processing::instructions::and_instruction_6::AndInstruction;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::equal_instruction_7::EqualInstruction;
use crate::processing::instructions::invert_instruction_1::InvertInstruction;
use crate::processing::instructions::jump_if_not_instruction_2::JumpIfNotInstruction;
use crate::processing::instructions::jump_instruction_3::JumpInstruction;
use crate::processing::instructions::jump_variable_instruction_4::JumpVariableInstruction;
use crate::processing::instructions::or_instruction_8::OrInstruction;
use crate::processing::instructions::print_chars_instruction_9::PrintCharsInstruction;
use crate::processing::instructions::print_instruction_5::PrintInstruction;

pub fn translate(data: &Vec<u8>) {
    println!("<------------------------------>");
    let mut i: usize = 0;
    while i < data.len() {
        print!("[{:0>5}] | ", i);

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
                (JumpIfNotInstruction::get_debug(&data[i..i+ JumpIfNotInstruction::get_size()]),
                 JumpIfNotInstruction::get_size())
            },
            3 => {
                (JumpInstruction::get_debug(&data[i..i+JumpInstruction::get_size()]),
                 JumpInstruction::get_size())
            },
            4 => {
                (JumpVariableInstruction::get_debug(&data[i..i+JumpVariableInstruction::get_size()]),
                 JumpVariableInstruction::get_size())
            },
            5 => {
                (PrintInstruction::get_debug(&data[i..i+PrintInstruction::get_size()]),
                 PrintInstruction::get_size())
            },
            6 => {
                (AndInstruction::get_debug(&data[i..i+AndInstruction::get_size()]),
                 AndInstruction::get_size())
            },
            7 => {
                (EqualInstruction::get_debug(&data[i..i+EqualInstruction::get_size()]),
                 EqualInstruction::get_size())
            },
            8 => {
                (OrInstruction::get_debug(&data[i..i+OrInstruction::get_size()]),
                 OrInstruction::get_size())
            },
            9 => {
                (PrintCharsInstruction::get_debug(&data[i..i+PrintCharsInstruction::get_size()]),
                 PrintCharsInstruction::get_size())
            },
            code => panic!("Debug not implemented for code {}", code),
        };

        println!("{}", output);

        i += size;
    }
    println!("<------------------------------>");
}