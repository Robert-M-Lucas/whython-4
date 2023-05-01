use crate::processing::blocks::BlockHandler;
use crate::processing::instructions::jump_if_not_instruction_2::JumpIfNotInstruction;
use crate::processing::instructions::jump_instruction_3::JumpInstruction;
use crate::processing::lines::arithmetic::handle_arithmetic_section;
use crate::processing::processor::MemoryManagers;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Block, Symbol};
use crate::processing::types::TypeSymbol;
use crate::propagate_error;

pub struct IfBlock {
    jump_next_instruction: Option<JumpIfNotInstruction>,
    jump_end_instructions: Vec<JumpInstruction>
}

impl IfBlock {
    pub fn new() -> Box<dyn BlockHandler> {
        Box::new(
            Self {
                jump_next_instruction: None,
                jump_end_instructions: Vec::new()
            }
        )
    }
}

impl BlockHandler for IfBlock {
    fn on_entry(&mut self, memory_managers: &mut MemoryManagers, reference_stack: &mut ReferenceStack, symbol_line: &Vec<Symbol>) -> Result<(), String> {
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

        self.jump_next_instruction = Some(JumpIfNotInstruction::new_alloc(memory_managers, condition_boolean, 0));

        Ok(())
    }

    fn on_exit(&mut self, memory_managers: &mut MemoryManagers, reference_stack: &mut ReferenceStack, symbol_line: &Vec<Symbol>) -> Result<bool, String> {
        fn exit_with_cleanup(this: &mut IfBlock, memory_managers: &mut MemoryManagers, reference_stack: &mut ReferenceStack ) -> Result<bool, String> {
            propagate_error!(this.on_forced_exit(memory_managers, reference_stack));
            return Ok(true);
        }

        if symbol_line.len() == 0 {
            return exit_with_cleanup(self, memory_managers, reference_stack);
        }

        let block_type = match &symbol_line[0] {
            Symbol::Block(block) => block,
            _ => {
                return exit_with_cleanup(self, memory_managers, reference_stack);
            }
        };

        match block_type {
            Block::Elif => {
                self.jump_end_instructions.push(JumpInstruction::new_alloc(memory_managers, 0));
                self.jump_next_instruction.as_mut().unwrap().set_destination(memory_managers, memory_managers.program_memory.get_position());
                propagate_error!(self.on_entry(memory_managers, reference_stack, symbol_line));
                reference_stack.remove_handler();
                reference_stack.add_handler();
                Ok(false)
            },
            Block::Else => {
                if symbol_line.len() > 1 { return Err("Else cannot be followed by any other symbol".to_string()); }
                self.jump_end_instructions.push(JumpInstruction::new_alloc(memory_managers, 0));
                self.jump_next_instruction.as_mut().unwrap().set_destination(memory_managers, memory_managers.program_memory.get_position());
                self.jump_next_instruction = None;
                reference_stack.remove_handler();
                reference_stack.add_handler();
                Ok(false)
            }
            _ => return exit_with_cleanup(self, memory_managers, reference_stack),
        }
    }

    fn on_forced_exit(&mut self, memory_managers: &mut MemoryManagers, _reference_stack: &mut ReferenceStack) -> Result<(), String> {
        /*
        If :: Jump to next if not
            content
             :: Jump to end

        ElIf :: Jump to next if not
            content
             :: Jump to end

         ElIf :: Jump to next if not
            content

         */

        // Set jump to next
        match self.jump_next_instruction.as_mut() {
            Some(instruction) => instruction.set_destination(memory_managers, memory_managers.program_memory.get_position()),
            None => {}
        }

        // Set all jump to end
        for j in self.jump_end_instructions.iter_mut() {
            j.set_destination(memory_managers, memory_managers.program_memory.get_position());
        }
        Ok(())
    }
}