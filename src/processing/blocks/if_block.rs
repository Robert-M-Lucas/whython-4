use crate::processing::blocks::BlockHandler;
use crate::processing::instructions::jump_if_instruction_2::JumpIfInstruction;
use crate::processing::lines::arithmetic::handle_arithmetic_section;
use crate::processing::processor::MemoryManagers;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::Symbol;
use crate::processing::types::TypeSymbol;

pub struct IfBlock {
    jump_instruction: Option<JumpIfInstruction>
}

impl IfBlock {
    pub fn new() -> Box<dyn BlockHandler> {
        Box::new(
            Self {
                jump_instruction: None
            }
        )
    }
}

impl BlockHandler for IfBlock {
    fn on_entry(&mut self, memory_managers: &mut MemoryManagers, reference_stack: &mut ReferenceStack, symbol_line: &Vec<Symbol>) -> Result<(), String> {
        let condition_boolean =
            match handle_arithmetic_section(memory_managers, reference_stack, &symbol_line[1..], None) {
                Err(e) => return Err(e),
                Ok(value) => value.unwrap()
            };

        if condition_boolean.get_type() != TypeSymbol::Boolean {
            return Err(format!("If expression must evaluate to {}", TypeSymbol::Boolean.get_name()));
        }

        self.jump_instruction = Some(JumpIfInstruction::new_alloc(memory_managers, condition_boolean, 0));

        Ok(())
    }

    fn on_exit(&mut self, memory_managers: &mut MemoryManagers, _reference_stack: &mut ReferenceStack, _symbol_line: &Vec<Symbol>) -> Result<bool, String> {
        self.jump_instruction.as_mut().unwrap().set_destination(memory_managers, memory_managers.program_memory.get_position());
        Ok(true)
    }

    fn on_forced_exit(&mut self, memory_managers: &mut MemoryManagers, _reference_stack: &mut ReferenceStack, _symbol_line: &Vec<Symbol>) -> Result<(), String> {
        self.jump_instruction.as_mut().unwrap().set_destination(memory_managers, memory_managers.program_memory.get_position());
        Ok(())
    }
}