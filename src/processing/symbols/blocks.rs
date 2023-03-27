use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone)]
pub enum Block {
    While,
    Loop,
    If,
    Elif,
    Else,
}

pub struct BlockSymbolHandler {}

impl SymbolHandler for BlockSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        match string.as_str() {
            "while" => Some(Symbol::Block(Block::While)),
            "loop" => Some(Symbol::Block(Block::Loop)),
            "if" => Some(Symbol::Block(Block::If)),
            "elif" => Some(Symbol::Block(Block::Elif)),
            "else" => Some(Symbol::Block(Block::Else)),
            _ => None,
        }
    }
}
