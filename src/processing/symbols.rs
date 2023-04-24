mod assigners;
mod blocks;
mod builtins;
mod literals;
mod operators;
mod punctuation;

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

pub use punctuation::Punctuation;
pub use punctuation::PunctuationSymbolHandler;


#[derive(PartialEq, Clone, strum_macros::Display)]
pub enum Symbol {
    Assigner(Assigner),
    Literal(Literal),
    Operator(Operator),
    ArithmeticBlock(Vec<Symbol>),
    Indexer(Box<Symbol>),
    List(Vec<Symbol>),
    Type(TypeSymbol),
    Block(Block),
    Builtin(Builtin),
    Punctuation(Punctuation),
    Name(String)
}


pub trait SymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol>;
}

pub fn get_symbol(string: &String) -> Option<Symbol> {
    AllSymbolHandler::get_symbol(string)
}

pub fn try_arithmetic_block_into_parameters(arithmetic_block: &Symbol) -> Result<Literal, String> {
    let list = match arithmetic_block {
        Symbol::ArithmeticBlock(list) => list,
        _ => panic!("Must be arithmetic block")
    };

    if list.len() == 0 {
        return Ok(Literal::ParameterList(Vec::new()));
    }

    let mut parameter_list: Vec<(TypeSymbol, String)> = Vec::new();

    let mut i: usize = 0;

    while i < list.len() {
        if list.len() - i == 1 {
            return Err("Parameters must be formatted ([Type] [Name] , [Type] [Name] , [...])".to_string());
        }

        let type_symbol = match list[i] {
            Symbol::Type(type_symbol) => type_symbol,
            _ => return Err("Parameters must be formatted ([Type] [Name] , [Type] [Name] , [...])".to_string())
        };

        let name = match &list[i + 1] {
            Symbol::Name(name) => name.clone(),
            _ => return Err("Parameters must be formatted ([Type] [Name] , [Type] [Name] , [...])".to_string())
        };

        if i + 2 < list.len() {
            match list[i + 2] {
                Symbol::Punctuation(punctuation) => {
                    #[allow(unreachable_patterns)] match punctuation {
                        Punctuation::ListSeparator => (),
                        _ => return Err("Parameters must be formatted ([Type] [Name] , [Type] [Name] , [...])".to_string())
                    }
                }
                _ => return Err("Parameters must be formatted ([Type] [Name] , [Type] [Name] , [...])".to_string()),
            }
        }

        parameter_list.push((type_symbol, name));

        i += 3;
    }

    Ok(Literal::ParameterList(parameter_list))
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
            .or_else(|| PunctuationSymbolHandler::get_symbol(string))
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
