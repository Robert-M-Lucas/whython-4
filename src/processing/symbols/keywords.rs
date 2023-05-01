use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone, strum_macros::Display)]
pub enum Keyword {
    Break,
}

pub struct KeywordSymbolHandler {}

impl SymbolHandler for KeywordSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        match string.as_str() {
            "break" => Some(Symbol::Keyword(Keyword::Break)),
            _ => None,
        }
    }
}
