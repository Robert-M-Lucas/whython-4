use crate::processing::processor::MemoryManagers;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Punctuation, Symbol};
use crate::processing::types::{get_type, get_type_from_literal, Type};

/// Returns Ok(Some(Type))/Err if to_overwrite is None. If to_overwrite is Some, returns Ok(None)/Err
pub fn handle_arithmetic_section(memory_managers: &mut MemoryManagers,
                                 reference_stack: &ReferenceStack,
                                 section: &[Symbol], to_overwrite: Option<&Type>,
                                 must_evaluate: bool)
                                 -> Result<Option<Type>, String> {

    fn get_formatting_error() -> String {
        "Operations must be formatted [LHS] [Operator] [RHS] or [Operator] [Operand] or [Value]"
            .to_string()
    }

    if section.len() > 3 || section.len() == 0 {
        return Err(get_formatting_error());
    }

    if section.len() == 3 {
        let operator = match section[1] {
            Symbol::Operator(op) => op,
            _ => return
                Err(get_formatting_error()
                .to_string())
        };

        let mut _lhs_holder = None;
        let lhs = match &section[0] {
            Symbol::Name(name) => {
                match reference_stack.get_variable(name) {
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
                match handle_arithmetic_section(memory_managers, reference_stack,
                                                symbols, None,
                                                true) {
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
                match reference_stack.get_variable(name) {
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
                match handle_arithmetic_section(memory_managers, reference_stack,
                                                symbols, None,
                                                true) {
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
            let result_type = match lhs.get_operation_type(&operator, Some(rhs)) {
                Err(e) => return Err(e),
                Ok(value) => value
            };

            let mut result = match get_type(&result_type, memory_managers) {
                Err(e) => return Err(e),
                Ok(value) => value
            };
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
        if matches!(section[1], Symbol::ArithmeticBlock(_)) {
            let name = match &section[0] {
                Symbol::Name(name) => name.clone(),
                _ => return Err("Only a function can be called".to_string())
            };

            let function = match reference_stack.get_variable(&name) {
                Err(e) => return Err(e),
                Ok(value) => value
            };

            let arguments = match &section[1] {
                Symbol::ArithmeticBlock(symbols) => symbols,
                _ => panic!()
            };

            let mut i: usize = 0;

            let mut argument_list = Vec::new();

            while i < arguments.len() {
                argument_list.push(match handle_arithmetic_section(memory_managers, reference_stack, &[arguments[i].clone()], None, true) {
                    Err(e) => return Err(e),
                    Ok(value) => value.unwrap()
                });

                i += 1;

                if i < arguments.len() {
                    #[allow(unreachable_patterns)] match arguments[i] {
                        Symbol::Punctuation(punctuation) => match punctuation {
                            Punctuation::ListSeparator => {},
                            _ => return
                                Err("Arguments must be formatted ([ARGUMENT] , [ARGUMENT] , [...]"
                                    .to_string())
                        },
                        _ => return Err("Arguments must be formatted ([ARGUMENT] , [ARGUMENT] , [...]"
                            .to_string())
                    }
                }
                i += 1
            }

            return match to_overwrite {
                Some(to_overwrite) => {
                    match function.call(memory_managers, argument_list.iter().collect(), Some(to_overwrite)) {
                        Err(e) => Err(e),
                        Ok(_) => Ok(None)
                    }
                },
                None => {
                    return if must_evaluate {
                        let return_type = match function.get_return_type() {
                            Err(e) => return Err(e),
                            Ok(value) => {
                                match get_type(&value, memory_managers) {
                                    Err(e) => return Err(e),
                                    Ok(value) => value
                                }
                            }
                        };

                        match function.call(memory_managers, argument_list.iter().collect(), Some(&return_type)) {
                            Err(e) => Err(e),
                            Ok(_) => Ok(Some(return_type))
                        }
                    }
                    else {
                        match function.call(memory_managers, argument_list.iter().collect(), None) {
                            Err(e) => Err(e),
                            Ok(_) => Ok(None)
                        }
                    }
                }
            }
        }
        else {
            let operator = match section[0] {
                Symbol::Operator(op) => op,
                _ => return
                    Err(get_formatting_error()
                        .to_string())
            };

            let mut _lhs_holder = None;
            let lhs = match &section[1] {
                Symbol::Name(name) => {
                    match reference_stack.get_variable(name) {
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
                    match handle_arithmetic_section(memory_managers, reference_stack,
                                                    symbols, None,
                                                    true) {
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
                let result_type = match lhs.get_operation_type(&operator, None) {
                    Err(e) => return Err(e),
                    Ok(value) => value
                };

                let mut result = match get_type(&result_type, memory_managers) {
                    Err(e) => return Err(e),
                    Ok(value) => value
                };
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
    }
    else {
        return match &section[0] {
            Symbol::Name(name) => {
                match reference_stack.get_variable(name) {
                    Err(e) => return Err(e),
                    Ok(value) => {
                        if to_overwrite.is_none() {
                            let object =
                                match get_type(&value.get_type(), memory_managers) {
                                    Err(e) => return Err(e),
                                    Ok(value) => value
                                };
                            match object.assign_clone(memory_managers, value) {
                                Err(e) => return Err(e),
                                Ok(_) => {}
                            }
                            Ok(Some(object))
                        }
                        else {
                            match to_overwrite.unwrap().assign_clone(memory_managers, value) {
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
                handle_arithmetic_section(memory_managers, reference_stack,
                                          symbols, to_overwrite,
                                          true)
            },
            _ => return Err("Only a name or literal can stand alone".to_string())
        }
    }
}
