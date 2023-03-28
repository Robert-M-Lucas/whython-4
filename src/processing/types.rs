mod boolean;

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

    fn static_assign_clone(&mut self, memory_managers: &mut MemoryManagers, to_clone: &Box<dyn Type>) -> Result<(), String>;

    fn static_assign_literal(&mut self, memory_managers: &mut MemoryManagers, literal: &Literal) -> Result<(), String>;

    fn get_type(&self) -> TypeSymbol;

    fn get_address(&self) -> usize;

    fn get_size(&self) -> usize;

    fn operate(&self, memory_managers: &MemoryManagers, operator: Operator,
               rhs: Box<dyn Type>, destination: Box<dyn Type>) -> Result<(), String>;
}
