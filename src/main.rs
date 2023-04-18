#![allow(dead_code)]

mod memory_manager;
mod processing;
mod errors;
mod translator;
mod execution;

use std::env;
use std::fs;
use std::io::{Read, stdin, stdout, Write};
use std::mem::size_of;
use std::time::Instant;
use processing::preprocessor::convert_to_symbols;
use processing::processor::process_symbols;
use crate::execution::execute;

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press enter to exit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    wrapped_main();
    pause();
}

fn wrapped_main() {
    let args: Vec<String> = env::args().collect();
    println!("Platform pointer length: {} [{}-bit]", size_of::<usize>(), size_of::<usize>() * 8);
    let input_file;
    if args.len() >= 2 { input_file = args[1].clone(); }
    else { input_file = "main.why".to_string() }

    let input = match fs::read_to_string(input_file) {
        Err(_) => { return println!("File read error"); }
        Ok(value) => value,
    };

    let start = Instant::now();
    let r = match convert_to_symbols(input) {
        Err(e) => {
            println!("Compilation (pre) failed [{:?}]:\n\t{}", start.elapsed(), e);
            return;
        },
        Ok(value) => value
    };

    println!("Compilation (pre) completed [{:?}]", start.elapsed());

    let start = Instant::now();
    let mut memory = match process_symbols(r) {
        Err(e) => {
            println!("Compilation (post) failed [{:?}]:\n\t{}", start.elapsed(), e);
            return;
        },
        Ok(value) => value
    };

    println!("Compilation (post) completed [{:?}]", start.elapsed());

    // translate(&m.program_memory.memory);

    memory.variable_memory.dump_bytes("VariableMemory".to_string());
    memory.program_memory.dump_bytes("ProgramMemory".to_string());

    match execute(&mut memory) {
        Err(e) => println!("Execution failed: {}", e),
        Ok(_) => {}
    };

    memory.variable_memory.dump_bytes("VariableMemory - Post execution".to_string());
}
