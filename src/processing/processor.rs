use crate::memory_manager::MemoryManager;
use crate::errors::create_line_error;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::lines::function_line::FunctionLine;
use crate::processing::lines::if_line::IfLine;
use crate::processing::lines::LineHandler;
use crate::processing::lines::variable_assignment_line::VariableAssignmentLine;
use crate::processing::lines::variable_initialisation_line::VariableInitialisationLine;
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

pub fn process_symbols(symbols: Vec<(usize, Vec<Symbol>)>) -> Result<MemoryManagers, String> {
    let mut memory_managers =  MemoryManagers {
        program_memory: MemoryManager::new(),
        variable_memory: MemoryManager::new()
    };

    let mut block_coordinator = BlockCoordinator::new();

    'line_iterator: for (line_index, line) in symbols.into_iter().enumerate() {
        if line.1.len() == 0 { continue; }

        let indentation = line.0;
        let symbol_line = line.1;

        if indentation > block_coordinator.get_indentation() {
            return create_line_error("Indentation to high".to_string(), line_index)
        }

        while block_coordinator.get_indentation() >= 1
            && indentation == block_coordinator.get_indentation() - 1 {

            if block_coordinator.get_indentation() >= 2
                && indentation <= block_coordinator.get_indentation() - 2 {
                let result = block_coordinator.force_exit_block_handler(&mut memory_managers, &symbol_line);
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
            VariableInitialisationLine::process_line(&symbol_line, &mut memory_managers, &mut block_coordinator)
            .or_else(|| VariableAssignmentLine::process_line(&symbol_line, &mut memory_managers, &mut block_coordinator))
            .or_else( || IfLine::process_line(&symbol_line, &mut memory_managers, &mut block_coordinator))
            .or_else( || FunctionLine::process_line(&symbol_line, &mut memory_managers, &mut block_coordinator))
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

    //TODO Handle exiting remaining blocks

    Ok(memory_managers)
}
