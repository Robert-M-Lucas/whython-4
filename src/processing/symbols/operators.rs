use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Product,
    Divide,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
    Or,
    And,
}

pub struct OperatorSymbolHandler {}

impl SymbolHandler for OperatorSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        match string.as_str() {
            "+"  => Some(Symbol::Operator(Operator::Add)),
            "-"  => Some(Symbol::Operator(Operator::Subtract)),
            "*"  => Some(Symbol::Operator(Operator::Product)),
            "/"  => Some(Symbol::Operator(Operator::Divide)),
            ">"  => Some(Symbol::Operator(Operator::Greater)),
            "<"  => Some(Symbol::Operator(Operator::Less)),
            ">=" => Some(Symbol::Operator(Operator::GreaterEqual)),
            "<=" => Some(Symbol::Operator(Operator::LessEqual)),
            "==" => Some(Symbol::Operator(Operator::Equal)),
            "!=" => Some(Symbol::Operator(Operator::NotEqual)),
            "|"  => Some(Symbol::Operator(Operator::Or)),
            "&"  => Some(Symbol::Operator(Operator::And)),
            _ => None,
        }
    }
}
