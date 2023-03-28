use crate::processing::processor::MemoryManagers;
use crate::processing::ReferenceManager;
use crate::processing::symbols::Symbol;
use crate::processing::types::Type;

pub trait BlockHandler {
    fn on_entry(&mut self, memory_managers: &MemoryManagers,
                                 symbol_line: &Vec<Symbol>) -> Result<(), String>;

    fn on_exit(&mut self, memory_managers: &MemoryManagers,
                                symbol_line: &Vec<Symbol>) -> Result<bool, String>;

    fn on_forced_exit(&mut self, memory_managers: &MemoryManagers,
                                symbol_line: &Vec<Symbol>) -> Result<(), String>;
}

pub struct BlockCoordinator {
    stack: Vec<Box<dyn BlockHandler>>,
    reference_stack: Vec<ReferenceManager>,
}

impl BlockCoordinator {
    pub fn new() -> Self { Self { stack: Vec::new(), reference_stack: vec![ReferenceManager::new()] } }

    pub fn add_handler(&mut self, handler: Box<dyn BlockHandler>, memory_managers: &MemoryManagers,
                       symbol_line: &Vec<Symbol>) -> Result<(), String> {
        self.stack.push(handler);
        self.reference_stack.push(ReferenceManager::new());
        return self.stack.last_mut().unwrap().on_entry(memory_managers, symbol_line);
    }

    pub fn on_exit(&mut self, memory_managers: &MemoryManagers,
                   symbol_line: &Vec<Symbol>)  -> Result<bool, String> {

        if self.stack.len() == 0 { panic!("Called on_exit when not BlockHandler exists on stack!") }
        let result =
            self.stack.last_mut().unwrap().on_exit(memory_managers, symbol_line);
        if result.is_ok() {
            if result.unwrap() == true {
                self.stack.pop();
                self.reference_stack.pop();
                return Ok(true);
            }
            return Ok(false);
        }
        result
    }

    pub fn on_forced_exit(&mut self, memory_managers: &MemoryManagers,
                          symbol_line: &Vec<Symbol>) -> Result<(), String> {
        if self.stack.len() == 0 { panic!("Called on_exit when not BlockHandler exists on stack!") }
        let result =
            self.stack.last_mut().unwrap().on_forced_exit(memory_managers, symbol_line);
        self.stack.pop();
        self.reference_stack.pop();
        result
    }

    pub fn get_indentation(&self) -> usize { self.stack.len() }

    pub fn register_variable(&mut self, variable: Box<dyn Type>, name: String) -> Result<(), String> {
        return self.reference_stack.last_mut().unwrap().register_variable(variable, name);
    }

    pub fn get_variable(&mut self, name: String) -> Option<&Box<dyn Type>> {
        let mut i = self.reference_stack.len() - 1;
        let mut reference_manager = &self.reference_stack[i];
        loop {
            let r = reference_manager.get_variable(name.clone());
            if r.is_some() { return r; }
            if i == 0 { break; }
            i -= 1;
            reference_manager = &self.reference_stack[i];
        }

        None
    }
}