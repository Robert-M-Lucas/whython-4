use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, TypeSymbol};
use crate::processing::types::{Type, TypeTrait};

pub struct FunctionType {}

impl FunctionType {
    pub(crate) fn create_empty() -> Self { Self {} }
}

impl TypeTrait for FunctionType {
    fn assign_clone(&self, _super: &Type, _memory_managers: &mut MemoryManagers, 
                    _to_clone: &Type) -> Result<(), String> {
        Err(format!("{} can't be assigned from other function", self.get_type().get_name()))
    }

    fn static_assign_literal(&self, _super: &Type, _memory_managers: &mut MemoryManagers,
                             _literal: &Literal) -> Result<(), String> {
        Err(format!("{} can't be assigned from literal", self.get_type().get_name()))
    }
    
    fn get_type(&self) -> TypeSymbol { TypeSymbol::Boolean }

    fn get_size(&self) -> usize { 0 }
}