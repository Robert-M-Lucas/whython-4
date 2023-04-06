use crate::processing::block_handler::BlockCoordinator;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::Symbol;
use crate::processing::types::{get_type, get_type_from_literal, Type};

/// Returns Ok(Some(Type))/Err if to_overwrite is None. If to_overwrite is Some, returns Ok(None)/Err
pub fn handle_arithmetic_section(memory_managers: &mut MemoryManagers,
                                 block_coordinator: &BlockCoordinator,
                                 section: &[Symbol], to_overwrite: Option<&Type>)
                                 -> Result<Option<Type>, String> {

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

        let mut _lhs_holder = None;
        let lhs = match &section[0] {
            Symbol::Name(name) => {
                match block_coordinator.get_variable(name) {
                    Err(e) => return Err(e),
                    Ok(value) => value
                }
            },
            Symbol::Literal(literal) => {
                let object = get_type_from_literal(&literal, memory_managers);
                match object.static_assign_literal(memory_managers, &literal) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
                _lhs_holder = Some(object);
                _lhs_holder.as_ref().unwrap()
            },
            Symbol::ArithmeticBlock(symbols) => {
                match handle_arithmetic_section(memory_managers, block_coordinator,
                                                symbols, None) {
                    Err(e) => return Err(e),
                    Ok(object) => {
                        _lhs_holder = Some(object.unwrap());
                        _lhs_holder.as_ref().unwrap()
                    }
                }
            },
            _ => return Err("LHS must be a Name, Literal or an operation within brackets"
                .to_string())
        };

        let mut _rhs_holder = None;
        let rhs = match &section[2] {
            Symbol::Name(name) => {
                match block_coordinator.get_variable(name) {
                    Err(e) => return Err(e),
                    Ok(value) => value
                }
            },
            Symbol::Literal(literal) => {
                let object = get_type_from_literal(&literal, memory_managers);
                match object.static_assign_literal(memory_managers, &literal) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
                _rhs_holder = Some(object);
                _rhs_holder.as_ref().unwrap()
            },
            Symbol::ArithmeticBlock(symbols) => {
                match handle_arithmetic_section(memory_managers, block_coordinator,
                                                symbols, None) {
                    Err(e) => return Err(e),
                    Ok(object) => {
                        _rhs_holder = Some(object.unwrap());
                        _rhs_holder.as_ref().unwrap()
                    }
                }
            },
            _ => return Err("RHS must be a Name, Literal or an operation within brackets"
                .to_string())
        };

        if to_overwrite.is_none() {
            let mut result = get_type(&lhs.get_type(), memory_managers);
            match lhs.operate(memory_managers, operator, Some(rhs), &mut result) {
                Err(e) => return Err(e),
                Ok(_) => {}
            }

            return Ok(Some(result));
        }
        else {
            match lhs.operate(memory_managers, operator, Some(rhs),
                              to_overwrite.unwrap()) {
                Err(e) => return Err(e),
                Ok(_) => {}
            }

            return Ok(None);
        }
    }
    else if section.len() == 2 {
        let operator = match section[0] {
            Symbol::Operator(op) => op,
            _ => return
                Err("Operations must be formatted [LHS] [Operator] [RHS] or [Operator] [Operand]"
                    .to_string())
        };

        let mut _lhs_holder = None;
        let lhs = match &section[1] {
            Symbol::Name(name) => {
                match block_coordinator.get_variable(name) {
                    Err(e) => return Err(e),
                    Ok(value) => value
                }
            },
            Symbol::Literal(literal) => {
                let object = get_type_from_literal(&literal, memory_managers);
                match object.static_assign_literal(memory_managers, &literal) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
                _lhs_holder = Some(object);
                _lhs_holder.as_ref().unwrap()
            },
            Symbol::ArithmeticBlock(symbols) => {
                match handle_arithmetic_section(memory_managers, block_coordinator,
                                                symbols, None) {
                    Err(e) => return Err(e),
                    Ok(object) => {
                        _lhs_holder = Some(object.unwrap());
                        _lhs_holder.as_ref().unwrap()
                    }
                }
            },
            _ => return Err("Operand must be a Name, Literal or an operation within brackets"
                .to_string())
        };

        if to_overwrite.is_none() {
            let mut result = get_type(&lhs.get_type(), memory_managers);
            match lhs.operate(memory_managers, operator, None, &mut result) {
                Err(e) => return Err(e),
                Ok(_) => {}
            }

            return Ok(Some(result));
        }
        else {
            match lhs.operate(memory_managers, operator,
                              None, to_overwrite.unwrap()) {
                Err(e) => return Err(e),
                Ok(_) => {}
            }

            return Ok(None);
        }
    }
    else {
        return match &section[0] {
            Symbol::Name(name) => {
                match block_coordinator.get_variable(name) {
                    Err(e) => return Err(e),
                    Ok(value) => {
                        if to_overwrite.is_none() {
                            let object =
                                get_type(&value.get_type(), memory_managers);
                            match object.static_assign_clone(memory_managers, value) {
                                Err(e) => return Err(e),
                                Ok(_) => {}
                            }
                            Ok(Some(object))
                        }
                        else {
                            match to_overwrite.unwrap().static_assign_clone(memory_managers, value) {
                                Err(e) => return Err(e),
                                Ok(_) => { }
                            };
                            Ok(None)
                        }
                    }
                }
            },
            Symbol::Literal(literal) => {
                if to_overwrite.is_none() {
                    let object = get_type_from_literal(&literal, memory_managers);
                    match object.static_assign_literal(memory_managers, &literal) {
                        Err(e) => return Err(e),
                        Ok(_) => { }
                    };
                    Ok(Some(object))
                }
                else {
                    match to_overwrite.unwrap().static_assign_literal(memory_managers, &literal) {
                        Err(e) => return Err(e),
                        Ok(_) => { }
                    };
                    Ok(None)
                }
            },
            Symbol::ArithmeticBlock(symbols) => {
                handle_arithmetic_section(memory_managers, block_coordinator,
                                          symbols, to_overwrite)
            },
            _ => return Err("Only a name or literal can stand alone".to_string())
        }
    }
}
