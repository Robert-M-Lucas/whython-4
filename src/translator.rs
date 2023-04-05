use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::invert_instruction_1::InvertInstruction;

pub fn translate(data: &Vec<u8>) {
    println!("---------------");
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
            }
            _ => panic!(),
        };

        println!("{}", output);

        i += size;
    }
    println!("---------------");
}