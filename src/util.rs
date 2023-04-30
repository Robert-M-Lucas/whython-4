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

#[macro_export] macro_rules! col_println {
    ($color: ident, $($arg:tt)*) => {
        {
            use colored::Colorize;
            println!("{}", format!($($arg)*).$color())
        }
    };
    (($($col_args:tt),*), $($arg:tt)*) => {
        {
            use colored::Colorize;
            println!("{}", format!($($arg)*)$(.$col_args())*)
        }
    };
}

#[macro_export] macro_rules! col_print {
    ($color: ident, $($arg:tt)*) => {
       {
           use colored::Colorize;
           print!("{}", format!($($arg)*).$color())
       }
    };
}

pub fn warn(warning: &str) {
    col_println!((yellow, bold), "[WARNING]: {}", warning);
}

pub fn info(info: &str) {
    col_println!((blue, bold), "[INFO]: {}", info);
}