use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq)]
pub enum Type {
    Int,
    Bool,
    Char,
}

pub struct TypeSymbolHandler {}

impl SymbolHandler for TypeSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        match string.as_str() {
            "int" => Some(Symbol::Type(Type::Int)),
            "bool" => Some(Symbol::Type(Type::Bool)),
            "char" => Some(Symbol::Type(Type::Char)),
            _ => None,
        }
    }
}
