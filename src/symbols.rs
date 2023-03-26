pub mod assigner;
use assigner::Assigner;

pub enum Symbol {
    Assigner(Assigner)
}

pub trait SymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol>;
}

pub struct ExampleSymbolHandler {}

impl SymbolHandler for ExampleSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        Some(Symbol::Assigner(Assigner::Setter))
    }
}
