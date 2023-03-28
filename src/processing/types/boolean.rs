use crate::processing::instructions::assign_instruction::AssignInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, Operator, TypeSymbol};
use crate::processing::types::Type;

pub struct BooleanType {
    address: Option<usize>,
    name: Option<String>
}

const BOOLEAN_FALSE: u8 = 0x00;
const BOOLEAN_TRUE: u8 = 0xFF;

impl BooleanType {
    pub(crate) fn create_empty() -> Self {
        Self { address: None, name: None }
    }
}

impl Type for BooleanType {
    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn get_name(&self) -> String {
        self.name.clone().unwrap()
    }

    fn static_assign_clone(&mut self, memory_managers: &mut MemoryManagers, to_clone: &Box<dyn Type>) -> Result<(), String> {
        return Err("Boolean cloning assignment not implemented".to_string())
    }

    fn static_assign_literal(&mut self, memory_managers: &mut MemoryManagers, literal: &Literal) -> Result<(), String> {
        let value: bool;
        match literal
        {
            Literal::BoolLiteral(boolean) => { value = *boolean },
            Literal::IntLiteral(integer) => {
                if *integer == 0 { value = false; }
                else if *integer == 1 { value = true; }
                else { return Err("Booleans can only be assigned IntLiterals '0' or '1'".to_string()) }
            }
            _ => { return Err("Literal type not supported for Boolean assignment".to_string()) }
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

        self.address = Some(address);

        Ok(())
    }

    fn get_type(&self) -> TypeSymbol { TypeSymbol::Bool }

    fn get_address(&self) -> usize { self.address.unwrap() }

    fn get_size(&self) -> usize { 1 }

    fn operate(&self, memory_managers: &MemoryManagers, operator: Operator, rhs: Box<dyn Type>, destination: Box<dyn Type>) -> Result<(), String> {
        return Err("No operators implemented for Boolean".to_string())
    }
}