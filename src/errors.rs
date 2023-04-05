use crate::processing::symbols::Operator;
use crate::processing::types::{Type, TypeSymbol};

/// Takes zero-indexed line
pub fn create_line_error<T>(error: String, line: usize) -> Result<T, String> {
    Err(format!("Line {}: {}", line + 1, error))
}

pub fn create_op_not_impl_error<T>(operator: Operator, lhs: TypeSymbol, rhs: Option<&Type>) -> Result<T, String> {
    match rhs {
        Some(rhs) => Err(format!("{} operator not implemented for {} and {}",
                                 operator.get_name(), lhs.get_name(), rhs.get_name())),
        None => Err(format!("{} operator not implemented for {}",
                            operator.get_name(), lhs.get_name())),
    }
}