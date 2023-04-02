mod boolean;

use crate::processing::instructions::assign_instruction::AssignInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, Operator, TypeSymbol};
use crate::processing::types::boolean::BooleanType;


pub fn get_type(type_symbol: &TypeSymbol) -> Box<dyn Type> {
    match type_symbol
    {
        TypeSymbol::Bool => { Box::new(BooleanType::create_empty()) },
        _ => panic!("Type not implemented!")
    }
}

pub trait Type {
    fn set_name(&mut self, name: String);

    fn get_name(&self) -> String;

    fn static_assign_clone(&mut self, memory_managers: &mut MemoryManagers, to_clone: &Box<dyn Type>) -> Result<(), String> {
        if self.get_type() != to_clone.get_type() {
            return Err(format!("Mismatching types for assignment: {} -> {}",
                               to_clone.get_type().get_name(), self.get_type().get_name()))
        }

        self.set_address(memory_managers.variable_memory.reserve(self.get_size()));
        AssignInstruction::new_alloc(memory_managers, to_clone.get_address(), self.get_address(), self.get_size());
        Ok(())
    }

    fn static_assign_literal(&mut self, memory_managers: &mut MemoryManagers, literal: &Literal) -> Result<(), String> {
        Err(format!("Assignment from literals not implemented for {}", self.get_type().get_name()))
    }

    fn get_type(&self) -> TypeSymbol;

    fn get_address(&self) -> usize;

    fn get_size(&self) -> usize;

    fn set_address(&mut self, address: usize);

    fn operate(&self, memory_managers: &MemoryManagers, operator: Operator,
               rhs: Option<Box<dyn Type>>, destination: Box<dyn Type>) -> Result<(), String> {
        if rhs.is_some() {
            return Err(format!("{} operator not supported between {} and {}",
                        operator.get_name(), self.get_type().get_name(), rhs.unwrap().get_type().get_name()))
        }
        else {
            return Err(format!("{} operator not supported for {}",
                               operator.get_name(), self.get_type().get_name()))
        }
    }
}
