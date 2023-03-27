#![allow(dead_code)]

mod memory_manager;
mod processing;
mod error;

use std::fs;
use processing::preprocessor::convert_to_symbols;
use processing::processor::process_symbols;
use debugless_unwrap::*;

fn main() {
    let input = fs::read_to_string("main.why").expect("IO error");

    let r = convert_to_symbols(input);
    if r.is_err() {
        let s = r.debugless_unwrap_err();
        println!("Compilation (pre) failed:\n\t{}", s);
        return;
    }

    let r = process_symbols(r.unwrap());
    if r.is_err() {
        let s = r.debugless_unwrap_err();
        println!("Compilation (post) failed:\n\t{}", s);
        return;
    }
}
