use crate::processing::symbols::Symbol::ArithmeticBlock;
use super::Operator;
use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone, strum_macros::Display)]
pub enum Assigner {
    Setter,
    AdditionSetter,
    SubtractionSetter,
    ProductSetter,
    DivisionSetter,
}

impl Assigner {
    pub fn get_equivalent(&self, lhs: Symbol, rhs: Vec<Symbol>) -> Vec<Symbol> {
        let equivalent = match self {
            Assigner::Setter => {
                return vec![ArithmeticBlock(rhs)];
            },
            Assigner::AdditionSetter => Operator::Add,
            Assigner::SubtractionSetter => Operator::Subtract,
            Assigner::ProductSetter => Operator::Product,
            Assigner::DivisionSetter => Operator::Divide
        };

        vec![lhs, Symbol::Operator(equivalent), ArithmeticBlock(rhs)]
    }
}

pub struct AssignerSymbolHandler {}

impl SymbolHandler for AssignerSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        match string.as_str() {
            "=" => Some(Symbol::Assigner(Assigner::Setter)),
            "+=" => Some(Symbol::Assigner(Assigner::AdditionSetter)),
            "-=" => Some(Symbol::Assigner(Assigner::SubtractionSetter)),
            "*=" => Some(Symbol::Assigner(Assigner::ProductSetter)),
            "/=" => Some(Symbol::Assigner(Assigner::DivisionSetter)),
            _ => None,
        }
    }
}
