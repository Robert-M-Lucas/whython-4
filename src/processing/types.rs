mod boolean;

use crate::errors::create_op_not_impl_error;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, Operator, Symbol, SymbolHandler};
use crate::processing::types::boolean::BooleanType;

pub fn get_type(type_symbol: &TypeSymbol, memory_managers: &mut MemoryManagers) -> Type {
    match type_symbol
    {
        TypeSymbol::Boolean => { Type::new(Box::new(BooleanType::create_empty()), memory_managers) },
        _ => panic!("Type not implemented!")
    }
}

pub fn get_type_from_literal(literal: &Literal, memory_managers: &mut MemoryManagers) -> Type {
    match literal
    {
        Literal::BoolLiteral(_) => {
            Type::new(Box::new(BooleanType::create_empty()), memory_managers)
        },
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
    address: usize,
}

impl Type {
    pub fn new(internal_type: Box<dyn TypeTrait>, memory_managers: &mut MemoryManagers) -> Self {
        let address = memory_managers.variable_memory.reserve(internal_type.get_size());

        Self {
            internal_type,
            name: None,
            address
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name)
    }

    pub fn get_name(&self) -> String {
        self.name.clone().unwrap()
    }

    pub fn static_assign_clone(&self, memory_managers: &mut MemoryManagers, to_clone: &Type) -> Result<(), String> {
        self.internal_type.static_assign_clone(self, memory_managers, to_clone)
    }

    pub fn static_assign_literal(&self, memory_managers: &mut MemoryManagers, literal: &Literal) -> Result<(), String> {
        self.internal_type.static_assign_literal(self, memory_managers, literal)
    }

    pub fn get_type(&self) -> TypeSymbol {
        self.internal_type.get_type()
    }

    pub fn get_address(&self) -> usize {
        self.address
    }

    pub fn get_size(&self) -> usize {
        self.internal_type.get_size()
    }

    pub fn operate(&self, memory_managers: &mut MemoryManagers, operator: Operator, rhs: Option<&Type>, destination: &Type) -> Result<(), String> {
        self.internal_type.operate(self, memory_managers, operator, rhs, destination)
    }
}

pub trait TypeTrait {
    fn static_assign_clone(&self, _super: &Type, memory_managers: &mut MemoryManagers, to_clone: &Type) -> Result<(), String> {
        if self.get_type() != to_clone.get_type() {
            return Err(format!("Mismatching types for assignment: {} -> {}",
                               to_clone.get_type().get_name(), self.get_type().get_name()))
        }

        CopyInstruction::new_alloc(memory_managers, to_clone.get_address(), _super.get_address(), self.get_size());

        Ok(())
    }

    fn static_assign_literal(&self, _super: &Type, memory_managers: &mut MemoryManagers, _literal: &Literal) -> Result<(), String> {
        Err(format!("Assignment from literals not implemented for {}", self.get_type().get_name()))
    }

    fn get_type(&self) -> TypeSymbol;

    fn get_size(&self) -> usize;

    fn operate(&self, lhs: &Type, memory_managers: &mut MemoryManagers, operator: Operator,
               rhs: Option<&Type>, _destination: &Type) -> Result<(), String> {
        create_op_not_impl_error(operator, self.get_type(), rhs)
    }
}
