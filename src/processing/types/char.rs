use crate::errors::create_op_not_impl_error;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::equal_instruction_7::EqualInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, Operator, TypeSymbol};
use crate::processing::types::{Type, TypeTrait};

pub struct CharType {}

impl CharType {
    pub(crate) fn create_empty() -> Self { Self {} }
}

impl TypeTrait for CharType {
    fn static_assign_literal(&self, _super: &Type, memory_managers: &mut MemoryManagers,
                             literal: &Literal) -> Result<(), String> {

        let value: u8;
        match literal
        {
            Literal::StringLiteral(string) => {
                if string.len() != 1 {
                    return Err("Chars can only be assigned from StringLiterals of length 1".to_string());
                }

                value = string.chars().nth(0).unwrap() as u8;
            }
            Literal::IntLiteral(integer) => {
                if *integer < 0 || *integer > 255 {
                    return Err("Char can be assigned from IntLiterals 0-255 only".to_string());
                }

                value = *integer as u8;
            },
            unhandled_literal => {
                return Err(format!("{} not supported for {} assignment",
                                        unhandled_literal.get_name(), self.get_type().get_name()))
            }
        }

        let constant_address = memory_managers.variable_memory.append_byte(value);

        CopyInstruction::new_alloc(memory_managers, constant_address,
                                   _super.get_address(), self.get_size());

        Ok(())
    }

    fn get_type(&self) -> TypeSymbol { TypeSymbol::Character }

    fn get_size(&self) -> usize { 1 }

    fn get_operation_type(&self, _lhs: &Type, operator: &Operator, rhs: Option<&Type>) -> Result<TypeSymbol, String> {
        if rhs.is_none() {
            return match operator {
                _ => create_op_not_impl_error(&operator, self.get_type(), rhs)
            };
        }

        match rhs.as_ref().unwrap().get_type() {
            TypeSymbol::Character => {},
            _ => return create_op_not_impl_error(&operator, self.get_type(), rhs)
        };

        match operator {
            Operator::Equal => {
                Ok(TypeSymbol::Boolean)
            },
            _ => create_op_not_impl_error(&operator, self.get_type(), rhs)
        }
    }

    fn operate(&self, lhs: &Type, memory_managers: &mut MemoryManagers, operator: Operator,
               rhs: Option<&Type>, destination: &Type) -> Result<(), String> {

        if rhs.is_none() {
            return match operator {
                _ => create_op_not_impl_error(&operator, self.get_type(), rhs)
            };
        }

        match rhs.as_ref().unwrap().get_type() {
            TypeSymbol::Character => {},
            _ => return create_op_not_impl_error(&operator, self.get_type(), rhs)
        };

        match operator {
            Operator::Equal => {
                EqualInstruction::new_alloc(memory_managers, lhs.get_address(), rhs.unwrap().get_address(), self.get_size(), destination.get_address());
                Ok(())
            },
            _ => create_op_not_impl_error(&operator, self.get_type(), rhs)
        }
    }

    fn clone(&self) -> Box<dyn TypeTrait> {
        Box::new(Self::create_empty())
    }
}