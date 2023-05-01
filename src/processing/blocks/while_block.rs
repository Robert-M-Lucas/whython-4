use crate::processing::blocks::BlockHandler;

use crate::processing::instructions::jump_if_not_instruction_2::JumpIfNotInstruction;
use crate::processing::instructions::jump_instruction_3::JumpInstruction;
use crate::processing::lines::arithmetic::handle_arithmetic_section;
use crate::processing::processor::MemoryManagers;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Symbol};
use crate::processing::types::{TypeSymbol};
use crate::propagate_error;


pub struct WhileBlock {
    jump_end_instruction: Option<JumpIfNotInstruction>,
    jump_end_instructions: Vec<JumpInstruction>,
    jump_start_instructions: Vec<JumpInstruction>,
    start_position: Option<usize>
}

impl WhileBlock {
    pub fn new() -> Box<dyn BlockHandler> {
        Box::new(
            Self {
                jump_end_instruction: None,
                jump_end_instructions: Vec::new(),
                jump_start_instructions: Vec::new(),
                start_position: None
            }
        )
    }
}

impl BlockHandler for WhileBlock {
    fn on_entry(&mut self, memory_managers: &mut MemoryManagers, reference_stack: &mut ReferenceStack, symbol_line: &Vec<Symbol>) -> Result<(), String> {
        // warn("While block still under development!");

        self.start_position = Some(memory_managers.program_memory.get_position());

        let condition_boolean =
            match handle_arithmetic_section(memory_managers, reference_stack, &symbol_line[1..], None, true) {
                Err(e) => return Err(e),
                Ok(value) =>  {
                    if value.is_none() { return Err("Section does not evaluate to a value".to_string()); }
                    value.unwrap()
                }
            };

        if condition_boolean.get_type() != TypeSymbol::Boolean {
            return Err(format!("If expression must evaluate to {}", TypeSymbol::Boolean));
        }

        self.jump_end_instruction = Some(JumpIfNotInstruction::new_alloc(memory_managers, condition_boolean, 0));

        Ok(())
    }

    fn on_break(&mut self, memory_managers: &mut MemoryManagers) -> Result<bool, String> {
        self.jump_end_instructions.push(JumpInstruction::new_alloc(memory_managers, 0));
        Ok(true)
    }

    fn on_continue(&mut self, memory_managers: &mut MemoryManagers) -> Result<bool, String> {
        self.jump_start_instructions.push(JumpInstruction::new_alloc(memory_managers, 0));
        Ok(true)
    }

    fn on_exit(&mut self, memory_managers: &mut MemoryManagers, reference_stack: &mut ReferenceStack, _symbol_line: &Vec<Symbol>) -> Result<bool, String> {
        propagate_error!(self.on_forced_exit(memory_managers, reference_stack));
        Ok(true)
    }

    fn on_forced_exit(&mut self, memory_managers: &mut MemoryManagers, _reference_stack: &mut ReferenceStack) -> Result<(), String> {
        JumpInstruction::new_alloc(memory_managers, self.start_position.unwrap());
        self.jump_end_instruction.as_mut().unwrap().set_destination(memory_managers, memory_managers.program_memory.get_position());
        for i in self.jump_end_instructions.iter_mut() {
            i.set_destination(memory_managers, memory_managers.program_memory.get_position());
        }
        for i in self.jump_start_instructions.iter_mut() {
            i.set_destination(memory_managers, self.start_position.unwrap());
        }
        Ok(())
    }
}