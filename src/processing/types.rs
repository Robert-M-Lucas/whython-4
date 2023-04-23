pub mod boolean;
pub mod function;
pub mod char;
pub mod pointer;

use crate::errors::create_op_not_impl_error;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, Operator, Symbol, SymbolHandler};
use crate::processing::types::boolean::BooleanType;
use crate::processing::types::char::CharType;
use crate::processing::types::pointer::PointerType;

pub fn get_type(type_symbol: &TypeSymbol, memory_managers: &mut MemoryManagers) -> Result<Type, String> {
    match type_symbol
    {
        TypeSymbol::Boolean => {
            Ok(Type::new(Box::new(BooleanType::create_empty()), memory_managers))
        },
        TypeSymbol::Character => {
            Ok(Type::new(Box::new(CharType::create_empty()), memory_managers))
        },
        TypeSymbol::Pointer => {
            Ok(Type::new(Box::new(PointerType::create_empty()), memory_managers))
        }
        type_symbol => Err(format!("{:?}(s) cannot be created! (Are you trying to operate on an invalid type?)", type_symbol))
    }
}

pub fn get_type_from_literal(literal: &Literal, memory_managers: &mut MemoryManagers) -> Type {
    match literal
    {
        Literal::BoolLiteral(_) => {
            Type::new(Box::new(BooleanType::create_empty()), memory_managers)
        },
        Literal::StringLiteral(_) => {
            Type::new(Box::new(CharType::create_empty()), memory_managers)
        },
        _ => panic!("Cannot infer type from {}", literal.get_name())
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TypeSymbol {
    Integer,
    Boolean,
    Character,
    Function,
    Pointer,
}

pub struct TypeSymbolHandler {}

impl SymbolHandler for TypeSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        match string.as_str() {
            "int" => Some(Symbol::Type(TypeSymbol::Integer)),
            "bool" => Some(Symbol::Type(TypeSymbol::Boolean)),
            "char" => Some(Symbol::Type(TypeSymbol::Character)),
            "ptr" => Some(Symbol::Type(TypeSymbol::Pointer)),
            _ => None,
        }
    }
}

impl TypeSymbol {
    pub(crate) fn get_name(&self) -> &str {
        return match self {
            TypeSymbol::Integer => "Integer",
            TypeSymbol::Boolean => "Boolean",
            TypeSymbol::Character => "Char",
            TypeSymbol::Function => "Function",
            TypeSymbol::Pointer => "Pointer",
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
        let address =
            memory_managers.variable_memory.reserve(internal_type.get_size());

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

    pub fn assign_clone(&self, memory_managers: &mut MemoryManagers,
                        to_clone: &Type) -> Result<(), String> {
        self.internal_type.assign_clone(self, memory_managers, to_clone)
    }

    pub fn static_assign_literal(&self, memory_managers: &mut MemoryManagers,
                                 literal: &Literal) -> Result<(), String> {
        self.internal_type.static_assign_literal(self, memory_managers, literal)
    }

    pub fn get_type(&self) -> TypeSymbol {
        self.internal_type.get_type()
    }

    pub fn get_return_type(&self) -> Result<TypeSymbol, String> {
        self.internal_type.get_return_type()
    }

    pub fn get_address(&self) -> usize {
        self.address
    }

    pub fn get_size(&self) -> usize {
        self.internal_type.get_size()
    }

    pub fn call(&self, memory_managers: &mut MemoryManagers, arguments: Vec<&Type>, destination: Option<&Type>) -> Result<(), String> {
        self.internal_type.call(memory_managers, arguments, destination)
    }

    pub fn get_operation_type(&self, operator: &Operator,
                   rhs: Option<&Type>) -> Result<TypeSymbol, String> {
        self.internal_type.get_operation_type(self,
                                   operator, rhs)
    }

    pub fn operate(&self, memory_managers: &mut MemoryManagers, operator: Operator,
                   rhs: Option<&Type>, destination: &Type) -> Result<(), String> {
        self.internal_type.operate(self, memory_managers, 
                                   operator, rhs, destination)
    }
    
    pub fn clone(&self) -> Self {
        Self {
            internal_type: self.internal_type.clone(),
            name: self.name.clone(),
            address: self.address,
        }
    }
}


pub trait TypeTrait {
    fn assign_clone(&self, _super: &Type, memory_managers: &mut MemoryManagers,
                    to_clone: &Type) -> Result<(), String> {
        if self.get_type() != to_clone.get_type() {
            return Err(format!("Mismatching types for assignment: {} -> {}",
                               to_clone.get_type().get_name(), self.get_type().get_name()))
        }

        CopyInstruction::new_alloc(memory_managers, to_clone.get_address(),
                                   _super.get_address(), self.get_size());

        Ok(())
    }

    fn static_assign_literal(&self, _super: &Type, _memory_managers: &mut MemoryManagers,
                             _literal: &Literal) -> Result<(), String> {
        Err(format!("Assignment from literals not implemented for {}", self.get_type().get_name()))
    }

    fn get_type(&self) -> TypeSymbol;

    fn get_return_type(&self) -> Result<TypeSymbol, String> {
        Err(format!("{} cannot be called", self.get_type().get_name()))
    }

    fn get_size(&self) -> usize;
    
    fn call(&self, _memory_managers: &mut MemoryManagers, _arguments: Vec<&Type>, _destination: Option<&Type>) -> Result<(), String> {
        Err(format!("{} cannot be called", self.get_type().get_name()))
    }

    fn get_operation_type(&self, _lhs: &Type, operator: &Operator,
                              rhs: Option<&Type>) -> Result<TypeSymbol, String> {
        create_op_not_impl_error(&operator, self.get_type(), rhs)
    }

    fn operate(&self, _lhs: &Type, _memory_managers: &mut MemoryManagers, operator: Operator,
               rhs: Option<&Type>, _destination: &Type) -> Result<(), String> {
        create_op_not_impl_error(&operator, self.get_type(), rhs)
    }
    
    fn clone(&self) -> Box<dyn TypeTrait>;
}
