use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone)]
pub enum Builtin {
    Print,
    Input,
}

pub struct BuiltinSymbolHandler {}

impl SymbolHandler for BuiltinSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        match string.as_str() {
            "print" => Some(Symbol::Builtin(Builtin::Print)),
            "input" => Some(Symbol::Builtin(Builtin::Input)),
            _ => None,
        }
    }
}
