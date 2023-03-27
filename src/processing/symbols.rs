mod assigners;
mod blocks;
mod builtins;
mod literals;
mod operators;
mod types;

pub use assigners::Assigner;
use assigners::AssignerSymbolHandler;

pub use literals::Literal;
use literals::LiteralSymbolHandler;
pub use literals::STRING_DELIMITERS;

pub use operators::Operator;
use operators::OperatorSymbolHandler;

pub use types::Type;
use types::TypeSymbolHandler;

pub use blocks::Block;
use blocks::BlockSymbolHandler;

pub use builtins::Builtin;
use builtins::BuiltinSymbolHandler;


#[derive(PartialEq)]
pub enum Symbol {
    Assigner(Assigner),
    Literal(Literal),
    Operator(Operator),
    Type(Type),
    Block(Block),
    Builtin(Builtin),
}


pub trait SymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol>;
}


pub fn get_symbol(string: &String) -> Option<Symbol> {
    AllSymbolHandler::get_symbol(string)
}


struct AllSymbolHandler {}

impl SymbolHandler for AllSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        AssignerSymbolHandler::get_symbol(string)
            .or_else(|| OperatorSymbolHandler::get_symbol(string))
            .or_else(|| TypeSymbolHandler::get_symbol(string))
            .or_else(|| BlockSymbolHandler::get_symbol(string))
            .or_else(|| BuiltinSymbolHandler::get_symbol(string))
            .or_else(|| LiteralSymbolHandler::get_symbol(string))
    }
}
