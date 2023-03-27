use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone)]
pub enum TypeSymbol {
    Int,
    Bool,
    Char,
}

pub struct TypeSymbolHandler {}

impl SymbolHandler for TypeSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        match string.as_str() {
            "int" => Some(Symbol::Type(TypeSymbol::Int)),
            "bool" => Some(Symbol::Type(TypeSymbol::Bool)),
            "char" => Some(Symbol::Type(TypeSymbol::Char)),
            _ => None,
        }
    }
}
