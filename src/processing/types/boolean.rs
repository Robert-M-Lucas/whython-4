use crate::errors::create_op_not_impl_error;
use crate::processing::instructions::and_instruction_6::AndInstruction;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::equal_instruction_7::EqualInstruction;
use crate::processing::instructions::invert_instruction_1::InvertInstruction;
use crate::processing::instructions::or_instruction_8::OrInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, Operator, TypeSymbol};
use crate::processing::types::{Type, TypeTrait};

pub struct BooleanType {}

pub const BOOLEAN_FALSE: u8 = 0x00;
pub const BOOLEAN_TRUE: u8 = 0xFF;

impl BooleanType {
    pub(crate) fn create_empty() -> Self { Self {} }
}

impl TypeTrait for BooleanType {
    fn static_assign_literal(&self, _super: &Type, memory_managers: &mut MemoryManagers,
                             literal: &Literal) -> Result<(), String> {

        let value: bool;
        match literal
        {
            Literal::BoolLiteral(boolean) => { value = *boolean },
            Literal::IntLiteral(integer) => {
                if *integer == 0 { value = false; }
                else if *integer == 1 { value = true; }
                else {
                    return Err(format!("{} can only be assigned {} '0' or '1'",
                                       self.get_type().get_name(),
                                       Literal::IntLiteral(0).get_name()))
                }
            }
            unhandled_literal => {
                return Err(format!("{} not supported for {} assignment",
                                        unhandled_literal.get_name(), self.get_type().get_name()))
            }
        }

        let constant_address;
        if value {
            constant_address = memory_managers.variable_memory.append_byte(BOOLEAN_TRUE); // Reserve for constant
        }
        else {
            constant_address = memory_managers.variable_memory.append_byte(BOOLEAN_FALSE); // Reserve for constant
        }

        CopyInstruction::new_alloc(memory_managers, constant_address,
                                   _super.get_address(), self.get_size());

        Ok(())
    }

    fn get_type(&self) -> TypeSymbol { TypeSymbol::Boolean }

    fn get_size(&self) -> usize { 1 }

    fn operate(&self, lhs: &Type, memory_managers: &mut MemoryManagers, operator: Operator,
               rhs: Option<&Type>, destination: &Type) -> Result<(), String> {

        if rhs.is_none() {
            return match operator {
                Operator::Not => {
                    InvertInstruction::new_alloc(memory_managers,
                                                 lhs.get_address(),
                                                 destination.get_address());
                    Ok(())
                },
                _ => create_op_not_impl_error(operator, self.get_type(), rhs)
            };
        }

        match operator {
            Operator::And => {
                AndInstruction::new_alloc(memory_managers, lhs.get_address(), rhs.unwrap().get_address(), destination.get_address());
                Ok(())
            },
            Operator::Equal => {
                EqualInstruction::new_alloc(memory_managers, lhs.get_address(), rhs.unwrap().get_address(), self.get_size(), destination.get_address());
                Ok(())
            },
            Operator::Or => {
                OrInstruction::new_alloc(memory_managers, lhs.get_address(), rhs.unwrap().get_address(), destination.get_address());
                Ok(())
            },
            _ => create_op_not_impl_error(operator, self.get_type(), rhs)
        }
    }

    fn clone(&self) -> Box<dyn TypeTrait> {
        Box::new(Self::create_empty())
    }
}