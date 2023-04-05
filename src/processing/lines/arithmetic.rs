use std::fs::read_to_string;
use crate::processing::block_handler::BlockCoordinator;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::Symbol;
use crate::processing::types::{get_type, get_type_from_literal, Type};

pub fn handle_arithmetic_section(memory_managers: &mut MemoryManagers,
                                 block_coordinator: &BlockCoordinator, section: &[Symbol])
                                 -> Result<Type, String> {

    if section.len() > 3 || section.len() == 0 {
        return Err("Operations must be formatted [LHS] [Operator] [RHS] or [Operator] [Operand] or [Value]"
            .to_string());
    }

    if section.len() == 3 {
        let operator = match section[1] {
            Symbol::Operator(op) => op,
            _ => return
                Err("Operations must be formatted [LHS] [Operator] [RHS] or [Operator] [Operand]"
                .to_string())
        };

        let mut lhs_holder = None;
        let lhs = match &section[0] {
            Symbol::Name(name) => {
                match block_coordinator.get_variable(name) {
                    Err(e) => return Err(e),
                    Ok(value) => value
                }
            },
            Symbol::Literal(literal) => {
                let mut object = get_type_from_literal(&literal);
                match object.static_assign_literal(memory_managers, &literal) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
                lhs_holder = Some(object);
                lhs_holder.as_ref().unwrap()
            },
            Symbol::ArithmeticBlock(symbols) => {
                match handle_arithmetic_section(memory_managers, block_coordinator, symbols) {
                    Err(e) => return Err(e),
                    Ok(object) => {
                        lhs_holder = Some(object);
                        lhs_holder.as_ref().unwrap()
                    }
                }
            },
            _ => return Err("LHS must be a Name, Literal or an operation within brackets".to_string())
        };

        let mut rhs_holder = None;
        let rhs = match &section[2] {
            Symbol::Name(name) => {
                match block_coordinator.get_variable(name) {
                    Err(e) => return Err(e),
                    Ok(value) => value
                }
            },
            Symbol::Literal(literal) => {
                let mut object = get_type_from_literal(&literal);
                match object.static_assign_literal(memory_managers, &literal) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
                rhs_holder = Some(object);
                rhs_holder.as_ref().unwrap()
            },
            Symbol::ArithmeticBlock(symbols) => {
                match handle_arithmetic_section(memory_managers, block_coordinator, symbols) {
                    Err(e) => return Err(e),
                    Ok(object) => {
                        rhs_holder = Some(object);
                        rhs_holder.as_ref().unwrap()
                    }
                }
            },
            _ => return Err("RHS must be a Name, Literal or an operation within brackets".to_string())
        };

        let mut result = get_type(&lhs.get_type());
        match lhs.operate(memory_managers, operator, Some(rhs), &mut result) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }

        return Ok(result);
    }
    else if section.len() == 2 {
        let operator = match section[0] {
            Symbol::Operator(op) => op,
            _ => return
                Err("Operations must be formatted [LHS] [Operator] [RHS] or [Operator] [Operand]"
                    .to_string())
        };

        let mut lhs_holder = None;
        let lhs = match &section[1] {
            Symbol::Name(name) => {
                match block_coordinator.get_variable(name) {
                    Err(e) => return Err(e),
                    Ok(value) => value
                }
            },
            Symbol::Literal(literal) => {
                let mut object = get_type_from_literal(&literal);
                match object.static_assign_literal(memory_managers, &literal) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
                lhs_holder = Some(object);
                lhs_holder.as_ref().unwrap()
            },
            Symbol::ArithmeticBlock(symbols) => {
                match handle_arithmetic_section(memory_managers, block_coordinator, symbols) {
                    Err(e) => return Err(e),
                    Ok(object) => {
                        lhs_holder = Some(object);
                        lhs_holder.as_ref().unwrap()
                    }
                }
            },
            _ => return Err("Operand must be a Name, Literal or an operation within brackets".to_string())
        };

        let mut result = get_type(&lhs.get_type());
        match lhs.operate(memory_managers, operator, None, &mut result) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }

        return Ok(result);
    }
    else {
        return match &section[0] {
            Symbol::Name(name) => {
                match block_coordinator.get_variable(name) {
                    Err(e) => return Err(e),
                    Ok(value) => {
                        let mut object = get_type(&value.get_type());
                        match object.static_assign_clone(memory_managers, value) {
                            Err(e) => return Err(e),
                            Ok(_) => { }
                        }
                        Ok(object)
                    }
                }
            },
            Symbol::Literal(literal) => {
                let mut object = get_type_from_literal(&literal);
                match object.static_assign_literal(memory_managers, &literal) {
                    Err(e) => return Err(e),
                    Ok(_) => { }
                };
                Ok(object)
            },
            Symbol::ArithmeticBlock(symbols) => {
                handle_arithmetic_section(memory_managers, block_coordinator, symbols)
            },
            _ => return Err("Only a name or literal can stand alone".to_string())
        }
    }
}
