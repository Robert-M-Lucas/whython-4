use std::mem::size_of;

pub fn get_u8(pointer: &usize, memory: &[u8]) -> u8 {
    u8::from_le_bytes((&memory[*pointer..(*pointer + 1)]).try_into().unwrap())
}

pub fn get_usize(pointer: &usize, memory: &[u8]) -> usize {
    usize::from_le_bytes((&memory[*pointer..(*pointer + size_of::<usize>())]).try_into().unwrap())
}

#[macro_export] macro_rules! propagate_error {
    ($result: expr) => {
        match $result {
            Err(e) => return Err(e),
            Ok(value) => value
        }
    };
}

pub fn warn(warning: &str) {
    println!("[WARNING]: {}", warning);
}