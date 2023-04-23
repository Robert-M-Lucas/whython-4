use crate::processing::types::TypeSymbol;
use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Clone)]
pub enum Literal {
    StringLiteral(String),
    IntLiteral(i32),
    BoolLiteral(bool),
    ParameterList(Vec<(TypeSymbol, String)>),
    None,
}

pub struct LiteralSymbolHandler {}

pub const STRING_DELIMITERS: [char; 2] = ['\'', '"'];

const ESCAPE_CODES: [(char, char); 1] = [('n', '\n')];

fn evaluate_string_escapes(input: String) -> String {
    let mut output = String::new();
    'char_loop: for c in input.chars() {
        for code in ESCAPE_CODES {
            if c == code.0 {
                output.push(code.1);
                continue 'char_loop;
            }
        }
        output.push(c);
    }
    output
}

impl SymbolHandler for LiteralSymbolHandler {
    fn get_symbol(string: &String) -> Option<Symbol> {
        (match string.as_str() {
            // Boolean
            "true" => Some(Symbol::Literal(Literal::BoolLiteral(true))),
            "false" => Some(Symbol::Literal(Literal::BoolLiteral(false))),
            "none" => Some(Symbol::Literal(Literal::None)),
            _ => None,
        })
        .or_else(
            // String
            || {
                if string.len() >= 2
                    && STRING_DELIMITERS.contains(&string.chars().nth(0).unwrap())
                    && string.chars().last().unwrap() == string.chars().nth(0).unwrap()
                {
                    return Some(Symbol::Literal(Literal::StringLiteral(
                        evaluate_string_escapes(string[1..string.len() - 1].to_string()),
                    )));
                }
                None
            },
        )
        .or_else(
            // Integer
            || match string.parse::<i32>() {
                Ok(ok) => Some(Symbol::Literal(Literal::IntLiteral(ok))),
                Err(_) => None,
            },
        )
    }
}

impl Literal {
    pub(crate) fn get_name(&self) -> &str {
        return match self {
            Literal::StringLiteral(_) => "StringLiteral",
            Literal::IntLiteral(_) => "IntLiteral",
            Literal::BoolLiteral(_) => "BoolLiteral",
            Literal::ParameterList(_) => "ParameterList",
            Literal::None => "None",
        }
    }
}
