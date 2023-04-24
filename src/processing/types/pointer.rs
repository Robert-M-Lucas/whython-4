use std::mem::size_of;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, TypeSymbol};
use crate::processing::types::{Type, TypeTrait};

pub struct PointerType {}

impl PointerType {
    pub(crate) fn create_empty() -> Self { Self {} }
}

impl TypeTrait for PointerType {
    fn static_assign_literal(&self, _super: &Type, memory_managers: &mut MemoryManagers,
                             literal: &Literal) -> Result<(), String> {
        let value: usize;

        match literal
        {
            Literal::IntLiteral(integer) => {
                value = match (*integer).try_into() {
                    Err(_) => return Err(format!("Cannot fit {}'s value '{}' into Pointer", literal.to_string(), integer)),
                    Ok(value) => value
                }
            },
            unhandled_literal => {
                return Err(format!("{} not supported for {} assignment",
                                   unhandled_literal.to_string(), self.get_type().to_string()))
            }
        }

        let constant_address = memory_managers.variable_memory.append(&value.to_le_bytes());

        CopyInstruction::new_alloc(memory_managers, constant_address,
                                   _super.get_address(), self.get_size());

        Ok(())
    }

    fn get_type(&self) -> TypeSymbol { TypeSymbol::Pointer }

    fn get_size(&self) -> usize { size_of::<usize>() }

    fn clone(&self) -> Box<dyn TypeTrait> {
        Box::new(Self::create_empty())
    }
}