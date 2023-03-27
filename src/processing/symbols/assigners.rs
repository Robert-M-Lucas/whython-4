use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone)]
pub enum Assigner {
    Setter,
    IncrementSetter,
    DecrementSetter,
    AdditionSetter,
    SubtractionSetter,
    ProductSetter,
    DivisionSetter,
}

pub struct AssignerSymbolHandler {}

impl SymbolHandler for AssignerSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        match string.as_str() {
            "=" => Some(Symbol::Assigner(Assigner::Setter)),
            "++" => Some(Symbol::Assigner(Assigner::IncrementSetter)),
            "--" => Some(Symbol::Assigner(Assigner::DecrementSetter)),
            "+=" => Some(Symbol::Assigner(Assigner::AdditionSetter)),
            "-=" => Some(Symbol::Assigner(Assigner::SubtractionSetter)),
            "*=" => Some(Symbol::Assigner(Assigner::ProductSetter)),
            "/=" => Some(Symbol::Assigner(Assigner::DivisionSetter)),
            _ => None,
        }
    }
}
