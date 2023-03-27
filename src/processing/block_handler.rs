use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::Symbol;

pub trait BlockHandler {
    fn on_entry(&mut self, memory_managers: &MemoryManagers,
                                 symbol_line: &Vec<Symbol>) -> Result<(), String>;

    fn on_exit(&mut self, memory_managers: &MemoryManagers,
                                symbol_line: &Vec<Symbol>) -> Result<bool, String>;

    fn on_forced_exit(&mut self, memory_managers: &MemoryManagers,
                                symbol_line: &Vec<Symbol>) -> Result<(), String>;
}

pub struct BlockCoordinator {
    stack: Vec<Box<dyn BlockHandler>>
}

impl BlockCoordinator {
    pub fn new() -> Self { Self { stack: Vec::new() } }

    pub fn add_handler(&mut self, handler: Box<dyn BlockHandler>, memory_managers: &MemoryManagers,
                       symbol_line: &Vec<Symbol>) -> Result<(), String> {
        self.stack.push(handler);
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
        result
    }

    pub fn get_indentation(&self) -> usize { self.stack.len() }
}