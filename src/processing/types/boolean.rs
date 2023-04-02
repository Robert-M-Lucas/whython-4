use crate::processing::instructions::assign_instruction::AssignInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, TypeSymbol};
use crate::processing::symbols::Literal::IntLiteral;
use crate::processing::types::TypeTrait;

pub struct BooleanType {}

const BOOLEAN_FALSE: u8 = 0x00;
const BOOLEAN_TRUE: u8 = 0xFF;

impl BooleanType {
    pub(crate) fn create_empty() -> Self { Self {} }
}

impl TypeTrait for BooleanType {
    fn static_assign_literal(&mut self, memory_managers: &mut MemoryManagers, literal: &Literal) -> Result<usize, String> {
        let value: bool;
        match literal
        {
            Literal::BoolLiteral(boolean) => { value = *boolean },
            Literal::IntLiteral(integer) => {
                if *integer == 0 { value = false; }
                else if *integer == 1 { value = true; }
                else {
                    return Err(format!("{} can only be assigned {} '0' or '1'",
                                          self.get_type().get_name(), IntLiteral(0).get_name()))
                }
            }
            unhandled_literal => {
                return Err(format!("{} not supported for {} assignment",
                                        unhandled_literal.get_name(), self.get_type().get_name()))
            }
        }

        let address = memory_managers.variable_memory.reserve(self.get_size());

        let constant_address;
        if value {
            constant_address = memory_managers.variable_memory.append_byte(BOOLEAN_TRUE); // Reserve for constant
        }
        else {
            constant_address = memory_managers.variable_memory.append_byte(BOOLEAN_FALSE); // Reserve for constant
        }

        AssignInstruction::new_alloc(memory_managers, constant_address, address, self.get_size());

        Ok(address)
    }

    fn get_type(&self) -> TypeSymbol { TypeSymbol::Boolean }

    fn get_size(&self) -> usize { 1 }
}