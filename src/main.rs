#![allow(dead_code)]

mod memory_manager;
mod processing;
mod errors;
mod translator;

use std::fs;
use std::mem::size_of;
use std::time::Instant;
use processing::preprocessor::convert_to_symbols;
use processing::processor::process_symbols;
use debugless_unwrap::*;
use crate::translator::translate;

fn main() {
    println!("Platform pointer length: {} [{}-bit]", size_of::<usize>(), size_of::<usize>() * 8);

    let input = fs::read_to_string("main.why");
    if input.is_err() {
        println!("File read error");
        return;
    }
    let input = input.unwrap();

    let start = Instant::now();
    let r = convert_to_symbols(input);
    if r.is_err() {
        let s = r.debugless_unwrap_err();
        println!("Compilation (pre) failed [{:?}]:\n\t{}", start.elapsed(), s);
        return;
    }
    println!("Compilation (pre) completed [{:?}]", start.elapsed());

    let start = Instant::now();
    let r = process_symbols(r.unwrap());
    if r.is_err() {
        let s = r.debugless_unwrap_err();
        println!("Compilation (post) failed [{:?}]:\n\t{}", start.elapsed(), s);
        return;
    }

    println!("Compilation (post) completed [{:?}]", start.elapsed());

    let m = r.unwrap();

    translate(&m.program_memory.memory);

    m.variable_memory.dump_bytes();
    m.program_memory.dump_bytes();
}
