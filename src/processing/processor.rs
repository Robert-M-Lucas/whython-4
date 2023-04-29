use std::fs;
use std::io::Write;
use std::mem::size_of;
use num_format::{Locale, ToFormattedString};
use crate::memory_manager::MemoryManager;
use crate::errors::create_line_error;
use crate::util::get_usize;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::lines::call_line::CallLine;
use crate::processing::lines::function_line::FunctionLine;
use crate::processing::lines::if_line::IfLine;
use crate::processing::lines::indexed_variable_assignment_line::IndexedVariableAssignmentLine;
use crate::processing::lines::LineHandler;
use crate::processing::lines::print_line::PrintLine;
use crate::processing::lines::variable_assignment_line::VariableAssignmentLine;
use crate::processing::lines::variable_initialisation_line::VariableInitialisationLine;
use crate::processing::lines::variable_initialisation_with_argument_line::VariableInitialisationWithArgumentLine;
use crate::processing::symbols::Symbol;

#[derive(PartialEq)]
pub enum ProcessingResult {
    Success,
    Unmatched,
    Failure(String)
}

impl ProcessingResult {
    pub fn or_else<F: FnOnce() -> ProcessingResult>(self, f: F) -> ProcessingResult
    {
        return match self {
            Self::Success | Self::Failure(_) => { self },
            Self::Unmatched => { f() }
        }
    }

    pub fn is_failure(&self) -> bool { return matches!(self, Self::Failure(_)); }
    pub fn is_success(&self) -> bool { return matches!(self, Self::Success); }
    pub fn is_unmatched(&self) -> bool { return matches!(self, Self::Unmatched); }

    pub fn get_error(self) -> String {
        return match self {
            Self::Failure(e) => { e },
            _ => panic!("Attempted to get error where there was none!")
        }
    }
}

pub struct MemoryManagers {
    pub program_memory: MemoryManager,
    pub variable_memory: MemoryManager,
}

impl MemoryManagers {
    pub fn save_to_compiled(&self, name: String) {
        let mut to_save = Vec::new();
        to_save.append(&mut Vec::from(self.variable_memory.get_position().to_le_bytes()));
        to_save.extend(&self.variable_memory.memory);
        to_save.extend(&self.program_memory.memory);

        let name = name + format!(" - {}.cwhy", size_of::<usize>() * 8).as_str();

        println!("Saving compiled data '{}' [{} bytes - {{{}:{}}}]",
                 &name, to_save.len().to_formatted_string(&Locale::en), self.variable_memory.get_position().to_formatted_string(&Locale::en)
                 , self.program_memory.get_position().to_formatted_string(&Locale::en));

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(name);

        if file.is_err() {
            println!("Failed to open file - {}", file.unwrap_err().to_string());
            return;
        }

        let mut file = file.unwrap();
        let r = file.write_all(&to_save);
        if r.is_err() { println!("Failed to write to file - {}", r.unwrap_err().to_string()) }
    }

    pub fn load_from_compiled(path: String) -> Result<Self, String> {
        println!("Loading precompiled data from file '{}'", &path);

        let data = match fs::read(path) {
            Err(e) => return Err(e.to_string()),
            Ok(value) => value
        };

        let variable_memory_length = get_usize(&0, &data);
        let mut variable_memory = Vec::with_capacity(variable_memory_length);
        let mut program_memory = Vec::with_capacity(data.len() - variable_memory_length - size_of::<usize>());

        for i in size_of::<usize>()..(size_of::<usize>() + variable_memory_length) {
            variable_memory.push(data[i])
        }

        for i in (size_of::<usize>() + variable_memory_length)..(data.len()) {
            program_memory.push(data[i])
        }

         Ok(Self { variable_memory: MemoryManager { memory: variable_memory }, program_memory: MemoryManager { memory: program_memory } })
    }
}

macro_rules! process_line {
    ($line: ident, $symbol_line: expr, $memory_managers: expr, $block_coordinator: expr) => {
        $line::process_line(&$symbol_line, &mut $memory_managers, &mut $block_coordinator)
    };
}

pub fn process_symbols(symbols: Vec<(usize, Vec<Symbol>)>) -> Result<MemoryManagers, String> {
    let mut memory_managers =  MemoryManagers {
        program_memory: MemoryManager::new(),
        variable_memory: MemoryManager::new()
    };

    let mut block_coordinator = BlockCoordinator::new();

    let line_count = symbols.len();

    'line_iterator: for (line_index, line) in symbols.into_iter().enumerate() {
        if line.1.len() == 0 { continue; }

        let indentation = line.0;
        let symbol_line = line.1;

        if indentation > block_coordinator.get_indentation() {
            return create_line_error("Indentation to high".to_string(), line_index)
        }

        while block_coordinator.get_indentation() >= 1
            && indentation <= block_coordinator.get_indentation() - 1 {

            if block_coordinator.get_indentation() >= 2
                && indentation <= block_coordinator.get_indentation() - 2 {
                let result = block_coordinator.force_exit_block_handler(&mut memory_managers);
                if result.is_err() { return create_line_error(result.unwrap_err(), line_index); }
            }
            else {
                let result = block_coordinator.exit_block_handler(&mut memory_managers, &symbol_line);
                if result.is_err() { return create_line_error(result.unwrap_err(), line_index); }
                if result.unwrap() == false { continue 'line_iterator; }
            }
        }
        // START


        let r =
            process_line!(VariableInitialisationWithArgumentLine, symbol_line, memory_managers, block_coordinator)
                .or_else( || process_line!(VariableInitialisationLine, symbol_line, memory_managers, block_coordinator))
                .or_else( || process_line!(CallLine, symbol_line, memory_managers, block_coordinator))
                .or_else( || process_line!(IndexedVariableAssignmentLine, symbol_line, memory_managers, block_coordinator))
                .or_else( || process_line!(VariableAssignmentLine, symbol_line, memory_managers, block_coordinator))
                .or_else( || process_line!(IfLine, symbol_line, memory_managers, block_coordinator))
                .or_else( || process_line!(FunctionLine, symbol_line, memory_managers, block_coordinator))
                .or_else( || process_line!(PrintLine, symbol_line, memory_managers, block_coordinator))
            ;


        // END
        if r.is_failure() {
            return create_line_error(r.get_error(), line_index);
        }
        else if r.is_unmatched() {
            return create_line_error("Line didn't match any known patterns".to_string(),
                                     line_index);
        }
    }

    //TODO: Don't duplicate code

    while block_coordinator.get_indentation() >= 1 {
        let result = block_coordinator.force_exit_block_handler(&mut memory_managers);
        if result.is_err() { return create_line_error(result.unwrap_err(), line_count); }
    }

    Ok(memory_managers)
}
