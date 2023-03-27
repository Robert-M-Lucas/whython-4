use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone)]
pub enum Builtin {
    Output,
    Input,
}

pub struct BuiltinSymbolHandler {}

impl SymbolHandler for BuiltinSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        match string.as_str() {
            "output" => Some(Symbol::Builtin(Builtin::Output)),
            "input" => Some(Symbol::Builtin(Builtin::Input)),
            _ => None,
        }
    }
}
