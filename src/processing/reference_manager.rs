use crate::processing::types::Type;

pub struct ReferenceStack {
    stack: Vec<ReferenceManager>,
}

impl ReferenceStack {
    pub fn new() -> Self {
        ReferenceStack { stack: vec![ReferenceManager::new()] }
    }

    pub fn register_variable(&mut self, variable: Type, name: String) -> Result<(), String> {
        return self.stack.last_mut().unwrap().register_variable(variable, name);
    }

    pub fn register_variable_with_offset(&mut self, variable: Type, name: String, offset: usize) -> Result<(), String> {
        let len = self.stack.len();
        self.stack[(len - 1) - offset].register_variable(variable, name)
    }

    pub fn get_variable(&self, name: &String) -> Result<&Type, String> {
        let mut i = self.stack.len() - 1;
        let mut reference_manager = &self.stack[i];
        loop {
            let r = reference_manager.get_variable(name.clone());
            if r.is_some() { return Ok(r.unwrap()); }
            if i == 0 { break; }
            i -= 1;
            reference_manager = &self.stack[i];
        }

        Err(format!("Variable '{}' not found", name))
    }

    pub fn add_handler(&mut self) { self.stack.push(ReferenceManager::new()); }
    
    pub fn remove_handler(&mut self) { self.stack.pop(); }

/*    pub fn start_handler_remove(&mut self) { self.stack_removed = Some(self.stack.pop().unwrap()); }

    pub fn cancel_handler_remove(&mut self) {
        self.stack.push(self.stack_removed.unwrap());
        self.stack_removed = None;
    }

    pub fn complete_handler_removal(&mut self) { self.stack_removed = None; }*/
}

pub struct ReferenceManager {
    variables: Vec<(Type, usize)> // Type, Array Index
}

impl ReferenceManager {
    pub fn new() -> Self {
        ReferenceManager { variables: Vec::new() }
    }

    pub fn register_variable(&mut self, mut variable: Type, name: String) -> Result<(), String> {
        if self.get_variable(name.clone()).is_some() {
            return Err(format!("Variable with name '{}' already exists", name));
        }
        variable.set_name(name);
        self.variables.push((variable, 0));
        Ok(())
    }

    pub fn register_variable_in_array(&mut self, mut variable: Type, name: String, index: usize) -> Result<(), String> {
        if self.get_variable(name.clone()).is_some() {
            return Err(format!("Variable with name '{}' already exists", name));
        }
        variable.set_name(name);
        self.variables.push((variable, index));
        Ok(())
    }

    pub fn get_variable(&self, name: String) -> Option<&Type> {
        for v in &self.variables {
            if *v.0.get_name() == name {
                return Some(&v.0);
            }
        }

        None
    }

    pub fn get_variable_with_index(&self, name: String, index: usize) -> Option<&Type> {
        for v in &self.variables {
            if *v.0.get_name() == name && v.1 == index {
                return Some(&v.0);
            }
        }

        None
    }
}