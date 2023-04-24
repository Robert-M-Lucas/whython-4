use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone, strum_macros::Display)]
pub enum Punctuation {
    ListSeparator
}

pub struct PunctuationSymbolHandler {}

impl SymbolHandler for PunctuationSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        match string.as_str() {
            "," => Some(Symbol::Punctuation(Punctuation::ListSeparator)),
            _ => None
        }
    }
}

// impl Punctuation {
//     pub(crate) fn get_name(&self) -> &str {
//         return match self {
//             Punctuation::ListSeparator => "ListSeparator"
//         }
//     }
// }
