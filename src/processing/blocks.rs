pub mod if_block;
pub mod function_block;
pub mod while_block;

use crate::processing::processor::MemoryManagers;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::Symbol;
use crate::processing::types::Type;
use crate::propagate_error;

pub trait BlockHandler {
    fn on_entry(&mut self, memory_managers: &mut MemoryManagers,
                block_coordinator: &mut ReferenceStack,
                symbol_line: &Vec<Symbol>) -> Result<(), String>;

    fn on_exit(&mut self, memory_managers: &mut MemoryManagers, reference_stack: &mut ReferenceStack,
               _symbol_line: &Vec<Symbol>) -> Result<bool, String> {
        propagate_error!(self.on_forced_exit(memory_managers, reference_stack));
        Ok(true)
    }

    fn on_forced_exit(&mut self, memory_managers: &mut MemoryManagers,
                      block_coordinator: &mut ReferenceStack) -> Result<(), String>;

    fn on_break(&mut self, _memory_managers: &mut MemoryManagers) -> Result<bool, String> {
        Ok(false)
    }

    fn on_continue(&mut self, _memory_managers: &mut MemoryManagers) -> Result<bool, String> {
        Ok(false)
    }
}

pub struct BlockCoordinator {
    stack: Vec<Box<dyn BlockHandler>>,
    reference_stack: ReferenceStack,
}

impl BlockCoordinator {
    pub fn new() -> Self {
        Self { stack: Vec::new(), reference_stack: ReferenceStack::new() }
    }

    pub fn add_block_handler(&mut self, mut handler: Box<dyn BlockHandler>, memory_managers: &mut MemoryManagers,
                             symbol_line: &Vec<Symbol>) -> Result<(), String> {

        self.reference_stack.add_handler();
        let r = handler.on_entry(memory_managers, self.get_reference_stack_mut(), symbol_line);
        self.stack.push(handler);
        r
    }

    pub fn break_block_handler(&mut self, memory_managers: &mut MemoryManagers) -> Result<(), String> {
        let mut success = false;
        for h in self.stack.iter_mut().rev() {
            if propagate_error!(h.on_break(memory_managers)) {
                success = true;
                break;
            }
        }

        if !success {
            return Err("None of the scopes 'break' is in support breaking".to_string());
        }
        Ok(())
    }

    pub fn continue_block_handler(&mut self, memory_managers: &mut MemoryManagers) -> Result<(), String> {
        let mut success = false;
        for h in self.stack.iter_mut().rev() {
            if propagate_error!(h.on_continue(memory_managers)) {
                success = true;
                break;
            }
        }

        if !success {
            return Err("None of the scopes 'continue' is in support continuing".to_string());
        }
        Ok(())
    }

    pub fn exit_block_handler(&mut self, memory_managers: &mut MemoryManagers,
                              symbol_line: &Vec<Symbol>)  -> Result<bool, String> {

        if self.stack.len() == 0 { panic!("Called on_exit when not BlockHandler exists on stack!") }

        let mut handler = self.stack.pop().unwrap();

        let result =
            handler.on_exit(memory_managers, self.get_reference_stack_mut(), symbol_line);


        if result.is_ok() {
            return if result.unwrap() == false {
                self.stack.push(handler);
                Ok(false)
            } else {
                self.reference_stack.remove_handler();
                Ok(true)
            }
        }
        result
    }

    pub fn force_exit_block_handler(&mut self, memory_managers: &mut MemoryManagers
        )  -> Result<(), String> {

        if self.stack.len() == 0 { panic!("Called on_exit when not BlockHandler exists on stack!") }

        let mut handler = self.stack.pop().unwrap();

        let result =
            handler.on_forced_exit(memory_managers, self.get_reference_stack_mut());

        result
    }

    pub fn get_indentation(&self) -> usize { self.stack.len() }

    pub fn get_reference_stack(&self) -> &ReferenceStack {
        &self.reference_stack
    }

    pub fn get_reference_stack_mut(&mut self) -> &mut ReferenceStack {
        &mut self.reference_stack
    }

    pub fn register_variable(&mut self, variable: Type, name: String) -> Result<(), String> {
        self.reference_stack.register_variable(variable, name)
    }

    pub fn get_variable(&self, name: &String) -> Result<&Type, String> {
        self.reference_stack.get_variable(name)
    }

    pub fn add_reference_handler(&mut self) { self.reference_stack.add_handler() }

    pub fn remove_reference_handler(&mut self) { self.reference_stack.remove_handler() }
}