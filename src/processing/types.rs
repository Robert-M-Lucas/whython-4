pub mod boolean;
pub mod function;
pub mod char;
pub mod pointer;

use crate::errors::create_op_not_impl_error;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::dynamic_from_copy_instruction_10::DynamicFromCopyInstruction;
use crate::processing::instructions::dynamic_to_copy_instruction_11::DynamicToCopyInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, Operator, Symbol, SymbolHandler};
use crate::processing::types::boolean::BooleanType;
use crate::processing::types::char::CharType;
use crate::processing::types::pointer::PointerType;
use crate::util::info;

macro_rules! create_type {
    ($internal_type: ident, $memory_managers: expr) => { Type::new(Box::new($internal_type::create_empty()), $memory_managers) };
}

pub fn get_type(type_symbol: &TypeSymbol, memory_managers: &mut MemoryManagers) -> Result<Type, String> {
    match type_symbol
    {
        TypeSymbol::Boolean => Ok(create_type!(BooleanType, memory_managers)),
        TypeSymbol::Character => Ok(create_type!(CharType, memory_managers)),
        TypeSymbol::Pointer => Ok(create_type!(PointerType, memory_managers)),
        type_symbol => Err(format!("{:?}(s) cannot be created! (Are you trying to operate on an invalid type?)", type_symbol))
    }
}

pub fn get_type_from_literal(literal: &Literal, memory_managers: &mut MemoryManagers) -> Result<Type, String> {
    match literal
    {
        Literal::BoolLiteral(_) => Ok(create_type!(BooleanType, memory_managers)),
        Literal::StringLiteral(_) => Ok(create_type!(CharType, memory_managers)),
        Literal::IntLiteral(_) => Ok(create_type!(PointerType, memory_managers)),
        _ => Err(format!("Cannot infer type from {}", literal))
    }
}

#[derive(PartialEq, Copy, Clone, Debug, strum_macros::Display)]
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

pub struct Type {
    internal_type: Box<dyn TypeTrait>,
    name: Option<String>,
    address: usize,
    indexed_len: Option<usize>,
}

impl Type {
    pub fn new(internal_type: Box<dyn TypeTrait>, memory_managers: &mut MemoryManagers) -> Self {
        let address =
            memory_managers.variable_memory.reserve(internal_type.get_size());

        Self {
            internal_type,
            name: None,
            address,
            indexed_len: None
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name)
    }

    pub fn get_name(&self) -> String {
        self.name.clone().unwrap()
    }

    pub fn is_indexed(&self) -> bool { self.indexed_len.is_some() }

    pub fn get_len(&self) -> usize { self.indexed_len.or_else(|| Some(1)).unwrap() }

    pub fn assign_clone(&self, memory_managers: &mut MemoryManagers,
                        to_clone: &Type) -> Result<(), String> {
        if self.is_indexed() {
            return Err("Tried to assign to type that needs indexing".to_string());
        }
        self.internal_type.assign_clone(self, memory_managers, to_clone)
    }

    pub fn static_assign_literal(&self, memory_managers: &mut MemoryManagers,
                                 literal: &Literal) -> Result<(), String> {
        if self.is_indexed() {
            return Err("Tried to assign to type that needs indexing".to_string());
        }
        self.internal_type.static_assign_literal(self, memory_managers, literal)
    }

    pub fn create_indexed(&mut self, _memory_managers: &mut MemoryManagers,
                      _argument_literal: &Literal, _assignment_literal: &Literal) -> Result<usize, String> {
        let result = self.internal_type.create_indexed(self, _memory_managers, _argument_literal, _assignment_literal);
        if result.is_err() {
            return result;
        }
        self.indexed_len = Some(result.as_ref().unwrap().clone());
        result
    }

    pub fn get_indexed(&self, memory_managers: &mut MemoryManagers,
                       index_pointer: &Type, destination: &Type) -> Result<(), String> {
        if !self.is_indexed() {
            return Err("Tried to index type that isn't indexed".to_string());
        }

        self.internal_type.get_index(self, memory_managers, index_pointer, destination)
    }

    pub fn set_indexed(&self, memory_managers: &mut MemoryManagers,
                   index_pointer: &Type, source: &Type) -> Result<(), String> {
        if !self.is_indexed() {
            return Err("Tried to index type that isn't indexed".to_string());
        }

        self.internal_type.set_index(self, memory_managers, index_pointer, source)
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
            indexed_len: self.indexed_len
        }
    }
}


pub trait TypeTrait {
    fn assign_clone(&self, _super: &Type, memory_managers: &mut MemoryManagers,
                    to_clone: &Type) -> Result<(), String> {
        if self.get_type() != to_clone.get_type() {
            return Err(format!("Mismatching types for assignment: {} -> {}",
                               to_clone.get_type().to_string(), self.get_type().to_string()))
        }

        CopyInstruction::new_alloc(memory_managers, to_clone.get_address(),
                                   _super.get_address(), self.get_size());

        Ok(())
    }

    fn static_assign_literal(&self, _super: &Type, _memory_managers: &mut MemoryManagers,
                             _literal: &Literal) -> Result<(), String> {
        Err(format!("Assignment from literals not implemented for {}", self.get_type().to_string()))
    }

    fn create_indexed(&self, _super: &Type, _memory_managers: &mut MemoryManagers,
                      _argument_literal: &Literal, _assignment_literal: &Literal) -> Result<usize, String> {
        Err(format!("{} cannot be created with initialisation argument", self.get_type().to_string()))
    }

    fn get_index(&self, _super: &Type, memory_managers: &mut MemoryManagers,
                      index_pointer: &Type, destination: &Type) -> Result<(), String> {

        match index_pointer.get_type() {
            TypeSymbol::Pointer => {},
            _ => return Err("Only pointers are supported for indexing this type".to_string())
        }

        if self.get_type() != destination.get_type() {
            return Err(format!("Cannot move value from indexed {} into {}", self.get_type(), destination.get_type()))
        }

        info(format!("{}", index_pointer.get_address()).as_str());

        DynamicFromCopyInstruction::new_alloc(memory_managers, _super.get_address(),
                                              self.get_size(), index_pointer.get_address(),
                                              destination.address, self.get_size());

        Ok(())
    }

    fn set_index(&self, _super: &Type, memory_managers: &mut MemoryManagers,
                 index_pointer: &Type, source: &Type) -> Result<(), String> {

        match index_pointer.get_type() {
            TypeSymbol::Pointer => {},
            _ => return Err("Only pointers are supported for indexing this type".to_string())
        }

        if self.get_type() != source.get_type() {
            return Err(format!("Cannot move value from {} into indexed {}", source.get_type(), self.get_type()))
        }

        DynamicToCopyInstruction::new_alloc(memory_managers, _super.get_address(),
                                              self.get_size(), index_pointer.get_address(),
                                              source.address, self.get_size());

        Ok(())
    }

    fn get_type(&self) -> TypeSymbol;

    fn get_return_type(&self) -> Result<TypeSymbol, String> {
        Err(format!("{} cannot be called", self.get_type().to_string()))
    }

    fn get_size(&self) -> usize;
    
    fn call(&self, _memory_managers: &mut MemoryManagers, _arguments: Vec<&Type>, _destination: Option<&Type>) -> Result<(), String> {
        Err(format!("{} cannot be called", self.get_type().to_string()))
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
