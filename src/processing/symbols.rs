mod assigners;
mod blocks;
mod builtins;
mod literals;
mod operators;

pub use assigners::Assigner;
use assigners::AssignerSymbolHandler;

pub use literals::Literal;
use literals::LiteralSymbolHandler;
pub use literals::STRING_DELIMITERS;

pub use operators::Operator;
use operators::OperatorSymbolHandler;

pub use super::types::TypeSymbol;
use super::types::TypeSymbolHandler;

pub use blocks::Block;
use blocks::BlockSymbolHandler;

pub use builtins::Builtin;
use builtins::BuiltinSymbolHandler;


#[derive(PartialEq, Clone)]
pub enum Symbol {
    Assigner(Assigner),
    Literal(Literal),
    Operator(Operator),
    ArithmeticBlock(Vec<Symbol>),
    Type(TypeSymbol),
    Block(Block),
    Builtin(Builtin),
    Name(String)
}


pub trait SymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol>;
}


pub fn get_symbol(string: &String) -> Option<Symbol> {
    AllSymbolHandler::get_symbol(string)
}

const ALLOWED_CHARS_IN_NAME: &str = "abcdefghijklmnopqrstuvwxyz_";

struct AllSymbolHandler {}

impl SymbolHandler for AllSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        AssignerSymbolHandler::get_symbol(string)
            .or_else(|| OperatorSymbolHandler::get_symbol(string))
            .or_else(|| TypeSymbolHandler::get_symbol(string))
            .or_else(|| BlockSymbolHandler::get_symbol(string))
            .or_else(|| BuiltinSymbolHandler::get_symbol(string))
            .or_else(|| LiteralSymbolHandler::get_symbol(string))
            .or_else(
                || {
                    for c in string.chars() {
                        if !ALLOWED_CHARS_IN_NAME.contains(c) { return None; }
                    }

                    return Some(Symbol::Name(string.clone()));
                }
            )
    }
}
