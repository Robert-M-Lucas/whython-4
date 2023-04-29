#![allow(dead_code)]

mod memory_manager;
mod processing;
mod errors;
mod translator;
mod execution;
pub mod util;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{Read, stdin, stdout, Write};
use std::mem::size_of;
use std::path::Path;
use std::time::Instant;
use processing::preprocessor::convert_to_symbols;
use processing::processor::process_symbols;
use crate::execution::execute;
use crate::processing::processor::MemoryManagers;
#[allow(unused_imports)]
use crate::translator::translate;

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
    println!("Platform pointer (usize) length: {} [{}-bit]", size_of::<usize>(), size_of::<usize>() * 8);
    let input_file;
    if args.len() >= 2 { input_file = args[1].clone(); }
    else { input_file = "main.why".to_string() }

    let mut memory;

    let extension = match Path::new(&input_file).extension().and_then(OsStr::to_str) {
        None => { println!("Invalid input file '{}'", input_file); return; },
        Some(value) => value
    };

    //? Compile
    if extension == "why" {
        let input = match fs::read_to_string(&input_file) {
            Err(e) => {
                println!("Error reading file '{}' - {}", input_file, e.to_string());
                return;
            }
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
        memory = match process_symbols(r) {
            Err(e) => {
                println!("Compilation (post) failed [{:?}]:\n\t{}", start.elapsed(), e);
                return;
            },
            Ok(value) => value
        };

        println!("Compilation (post) completed [{:?}]", start.elapsed());

        memory.save_to_compiled("Compiled".to_string());
    }
    //? Load compiled file
    else if extension == "cwhy" {
        memory = match MemoryManagers::load_from_compiled(input_file) {
            Err(e) => { println!("Loading precompiled file failed - {}", e); return; },
            Ok(value) => value
        };
    }
    else {
        println!("Unrecognised extension '{}'", extension);
        return;
    }

    //? translate(&memory.program_memory.memory);

    //? memory.variable_memory.dump_bytes("VariableMemory".to_string());
    //? memory.program_memory.dump_bytes("ProgramMemory".to_string());


    match execute(&mut memory) {
        Err(e) => println!("Execution failed: {}", e),
        Ok(_) => {}
    };

    //? memory.variable_memory.dump_bytes("VariableMemory - post".to_string());
}
