use crate::processing::block_handler::BlockCoordinator;
use crate::processing::processor::MemoryManagers;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::{Assigner, Literal, Symbol, TypeSymbol};
use crate::processing::types::get_type;
use super::LineHandler;

pub struct VariableInitialisationLine {}

impl LineHandler for VariableInitialisationLine {
    fn process_line(line: &Vec<Symbol>, memory_managers: &mut MemoryManagers, block_coordinator: &mut BlockCoordinator) -> ProcessingResult {
        if line.len() == 0 || !matches!(line[0], Symbol::Type(_)) { return ProcessingResult::Unmatched; }

        if line.len() < 2 || !matches!(line[1], Symbol::Name(_)) {
            return ProcessingResult::Failure("Type must be followed by a Name to initialise a variable".to_string());
        }

        if line.len() < 3 || match &line[2] {
            Symbol::Assigner(assigner) => {
                match assigner {
                    Assigner::Setter => false,
                    _ => true
                }
            },
            _ => true
        } {
            return ProcessingResult::Failure("Name must be followed by an '=' to initialise a variable".to_string());
        }

        if line.len() < 4 || (!matches!(line[3], Symbol::Name(_)) && !matches!(line[3], Symbol::Literal(_))) {
            return ProcessingResult::Failure("'=' must be followed by a Name or Literal to initialise a variable".to_string());
        }

        if line.len() > 5 { return ProcessingResult::Failure("Too many symbols on line".to_string()); }

        if matches!(line[3], Symbol::Name(_)) { return ProcessingResult::Failure("Assigning from name not implemented".to_string()); }

        let mut type_ = match line[0] {
            Symbol::Type(type_symbol) => get_type(&type_symbol),
            _ => panic!()
        };

        let r =
            type_.static_assign_literal(memory_managers,
            match &line[3] {
                Symbol::Literal(literal) => literal,
                _ => panic!()
            });

        if r.is_err() { return ProcessingResult::Failure(r.unwrap_err()); }

        let name = match &line[1] {
            Symbol::Name(name) => name,
            _ => panic!()
        };

        let r = block_coordinator.register_variable(type_, name.clone());

        if r.is_err() { return ProcessingResult::Failure(r.unwrap_err()); }

        ProcessingResult::Success
    }
}