use crate::processing::types::Type;

pub struct ReferenceManager {
    variables: Vec<Type>
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
        self.variables.push(variable);
        Ok(())
    }

    pub fn get_variable(&self, name: String) -> Option<&Type> {
        for v in &self.variables {
            if *v.get_name() == name {
                return Some(v);
            }
        }

        None
    }
}