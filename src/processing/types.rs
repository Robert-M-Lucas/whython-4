mod boolean;

use crate::processing::instructions::assign_instruction::AssignInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, Operator, Symbol, SymbolHandler};
use crate::processing::types::boolean::BooleanType;

pub fn get_type(type_symbol: &TypeSymbol) -> Type {
    match type_symbol
    {
        TypeSymbol::Boolean => { Type::new(Box::new(BooleanType::create_empty())) },
        _ => panic!("Type not implemented!")
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum TypeSymbol {
    Integer,
    Boolean,
    Character,
}

pub struct TypeSymbolHandler {}

impl SymbolHandler for TypeSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        match string.as_str() {
            "int" => Some(Symbol::Type(TypeSymbol::Integer)),
            "bool" => Some(Symbol::Type(TypeSymbol::Boolean)),
            "char" => Some(Symbol::Type(TypeSymbol::Character)),
            _ => None,
        }
    }
}

impl TypeSymbol {
    pub(crate) fn get_name(&self) -> &str {
        return match self {
            TypeSymbol::Integer => "Integer",
            TypeSymbol::Boolean => "Boolean",
            TypeSymbol::Character => "Char"
        }
    }
}

pub struct Type {
    internal_type: Box<dyn TypeTrait>,
    name: Option<String>,
    address: Option<usize>,
}

impl Type {
    pub fn new(internal_type: Box<dyn TypeTrait>) -> Self {
        Self {
            internal_type,
            name: None,
            address: None
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name)
    }

    pub fn get_name(&self) -> String {
        self.name.clone().unwrap()
    }

    pub fn static_assign_clone(&mut self, memory_managers: &mut MemoryManagers, to_clone: &Type) -> Result<(), String> {
        self.address = Some(match self.internal_type.static_assign_clone(memory_managers, to_clone) {
            Ok(address) => address,
            Err(e) => return Err(e)
        });

        Ok(())
    }

    pub fn static_assign_literal(&mut self, memory_managers: &mut MemoryManagers, literal: &Literal) -> Result<(), String> {
        self.address = Some(match self.internal_type.static_assign_literal(memory_managers, literal) {
            Ok(address) => address,
            Err(e) => return Err(e)
        });

        Ok(())
    }

    pub fn get_type(&self) -> TypeSymbol {
        self.internal_type.get_type()
    }

    pub fn get_address(&self) -> usize {
        self.address.unwrap()
    }

    pub fn get_size(&self) -> usize {
        self.internal_type.get_size()
    }

    pub fn operate(&self, memory_managers: &MemoryManagers, operator: Operator, rhs: Option<Box<dyn TypeTrait>>, destination: Box<dyn TypeTrait>) -> Result<(), String> {
        self.internal_type.operate(memory_managers, operator, rhs, destination)
    }
}

pub trait TypeTrait {
    fn static_assign_clone(&mut self, memory_managers: &mut MemoryManagers, to_clone: &Type) -> Result<usize, String> {
        if self.get_type() != to_clone.get_type() {
            return Err(format!("Mismatching types for assignment: {} -> {}",
                               to_clone.get_type().get_name(), self.get_type().get_name()))
        }

        let address = memory_managers.variable_memory.reserve(self.get_size());

        AssignInstruction::new_alloc(memory_managers, to_clone.get_address(), address, self.get_size());

        Ok(address)
    }

    fn static_assign_literal(&mut self, _memory_managers: &mut MemoryManagers, _literal: &Literal) -> Result<usize, String> {
        Err(format!("Assignment from literals not implemented for {}", self.get_type().get_name()))
    }

    fn get_type(&self) -> TypeSymbol;

    fn get_size(&self) -> usize;

    fn operate(&self, _memory_managers: &MemoryManagers, operator: Operator,
               rhs: Option<Box<dyn TypeTrait>>, _destination: Box<dyn TypeTrait>) -> Result<(), String> {
        return if rhs.is_some() {
            Err(format!("{} operator not supported between {} and {}",
                        operator.get_name(), self.get_type().get_name(), rhs.unwrap().get_type().get_name()))
        } else {
            Err(format!("{} operator not supported for {}",
                        operator.get_name(), self.get_type().get_name()))
        }
    }
}
