use crate::processing::processor::MemoryManagers;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Punctuation, Symbol};
use crate::processing::types::{get_type, get_type_from_literal, Type};
use crate::propagate_error;

/// Returns Ok(Some(Type))/Err if to_overwrite is None. If to_overwrite is Some, returns Ok(None)/Err
pub fn handle_arithmetic_section(memory_managers: &mut MemoryManagers,
                                 reference_stack: &ReferenceStack,
                                 section: &[Symbol], to_overwrite: Option<&Type>,
                                 must_evaluate: bool)
                                 -> Result<Option<Type>, String> {

    fn get_formatting_error() -> String {
        "Operations must be formatted:\n\
            \t[LHS] [Operator] [RHS] or\n\
            \t[Operator] [LHS] or\n\
            \t[Value] or\n\
            \t[Name][Index] or\n\
            \t[Function Name][Arguments]"
            .to_string()
    }

    if section.len() > 3 || section.len() == 0 {
        return Err(get_formatting_error());
    }

    //? Three operators - [LHS] [Operator] [RHS]
    if section.len() == 3 {
        // Get operator
        let operator = match section[1] {
            Symbol::Operator(op) => op,
            _ => return
                Err(get_formatting_error()
                .to_string())
        };

        // Get lhs
        let mut _lhs_holder = None;
        let lhs = match &section[0] {
            Symbol::Name(name) => {
                propagate_error!(reference_stack.get_variable(name))
            },
            Symbol::Literal(literal) => {
                let object = propagate_error!(get_type_from_literal(&literal, memory_managers));
                propagate_error!(object.static_assign_literal(memory_managers, &literal));
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

        // Get rhs
        let mut _rhs_holder = None;
        let rhs = match &section[2] {
            Symbol::Name(name) => {
                propagate_error!(reference_stack.get_variable(name))
            },
            Symbol::Literal(literal) => {
                let object = match get_type_from_literal(&literal, memory_managers) {
                    Err(e) => return Err(e),
                    Ok(o) => o
                };
                propagate_error!(object.static_assign_literal(memory_managers, &literal));
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

        // Return result
        if to_overwrite.is_none() {
            let result_type = propagate_error!(lhs.get_operation_type(&operator, Some(rhs)));

            let mut result = propagate_error!(get_type(&result_type, memory_managers));
            propagate_error!(lhs.operate(memory_managers, operator, Some(rhs), &mut result));

            return Ok(Some(result));
        }
        else {
            propagate_error!(lhs.operate(memory_managers, operator, Some(rhs),
                              to_overwrite.unwrap()));

            return Ok(None);
        }
    }
    //? Two symbols - function calling, indexing or prefix operators
    else if section.len() == 2 {
        //? Function call
        if matches!(section[0], Symbol::Name(_)) && matches!(section[1], Symbol::ArithmeticBlock(_)) {
            // Get function
            let name = match &section[0] {
                Symbol::Name(name) => name.clone(),
                _ => panic!()
            };

            let function = propagate_error!(reference_stack.get_variable(&name));

            // Get arguments
            let arguments = match &section[1] {
                Symbol::ArithmeticBlock(symbols) => symbols,
                _ => panic!()
            };

            let mut i: usize = 0;

            let mut argument_list = Vec::new();

            while i < arguments.len() {
                argument_list.push(propagate_error!(
                    handle_arithmetic_section(memory_managers, reference_stack, &[arguments[i].clone()], None, true)).unwrap());

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
                        // Call function with created destination
                        let return_type = match function.get_return_type() {
                            Err(e) => return Err(e),
                            Ok(value) => {
                                propagate_error!(get_type(&value, memory_managers))
                            }
                        };

                        match function.call(memory_managers, argument_list.iter().collect(), Some(&return_type)) {
                            Err(e) => Err(e),
                            Ok(_) => Ok(Some(return_type))
                        }
                    }
                    else {
                        // Call function without handling return
                        match function.call(memory_managers, argument_list.iter().collect(), None) {
                            Err(e) => Err(e),
                            Ok(_) => Ok(None)
                        }
                    }
                }
            }
        }
        //? Indexing
        else if matches!(section[1], Symbol::Indexer(_)) {
            // Get variable
            let to_index = match &section[0] {
                Symbol::Name(name) => propagate_error!(reference_stack.get_variable(name)),
                _ => return Err("Only a Name can be indexed".to_string())
            };

            // Index
            #[allow(unused_assignments)]
            let mut type_holder = None;
            let index = match &section[1] {
                Symbol::Indexer(symbol) => {
                    match symbol.as_ref() {
                        Symbol::Name(name) => propagate_error!(reference_stack.get_variable(name)),
                        Symbol::Literal(literal) => match get_type_from_literal(literal, memory_managers) {
                            Err(e) => return Err(e),
                            Ok(value) => {
                                propagate_error!(value.static_assign_literal(memory_managers, literal));
                                type_holder = Some(value);
                                type_holder.as_ref().unwrap()
                            }
                        },
                        _ => return Err("Name can only be indexed by a Name or a Literal".to_string())
                    }
                },
                _ => panic!()
            };

            if to_overwrite.is_some() {
                return match to_index.get_indexed(memory_managers, index, &to_overwrite.unwrap()) {
                    Err(e) => return Err(e),
                    Ok(_) => Ok(None)
                };
            }
            else {
                let return_type = propagate_error!(get_type(&to_index.get_type(), memory_managers));

                return match to_index.get_indexed(memory_managers, index, &return_type) {
                    Err(e) => return Err(e),
                    Ok(_) => Ok(Some(return_type))
                };
            }
        }
        //? Prefix operator e.g. '!a'
        else {
            // Get operator
            let operator = match section[0] {
                Symbol::Operator(op) => op,
                _ => return
                    Err(get_formatting_error()
                        .to_string())
            };

            // Get operand
            let mut _lhs_holder = None;
            let lhs = match &section[1] {
                Symbol::Name(name) => {
                    propagate_error!(reference_stack.get_variable(name))
                },
                Symbol::Literal(literal) => {
                    let object = propagate_error!(get_type_from_literal(&literal, memory_managers));
                    propagate_error!(object.static_assign_literal(memory_managers, &literal));
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

            // Return
            if to_overwrite.is_none() {
                let result_type = propagate_error!(lhs.get_operation_type(&operator, None));

                let mut result = propagate_error!(get_type(&result_type, memory_managers));
                propagate_error!(lhs.operate(memory_managers, operator, None, &mut result));

                return Ok(Some(result));
            }
            else {
                propagate_error!(lhs.operate(memory_managers, operator,
                                  None, to_overwrite.unwrap()));

                return Ok(None);
            }
        }
    }
    //? One symbol
    else {
        return match &section[0] {
            // Get type out of name
            Symbol::Name(name) => {
                match reference_stack.get_variable(name) {
                    Err(e) => return Err(e),
                    Ok(value) => {
                        if to_overwrite.is_none() {
                            let object =
                                propagate_error!(get_type(&value.get_type(), memory_managers));
                            propagate_error!(object.assign_clone(memory_managers, value));
                            Ok(Some(object))
                        }
                        else {
                            propagate_error!(to_overwrite.unwrap().assign_clone(memory_managers, value));
                            Ok(None)
                        }
                    }
                }
            },
            // Get type out of literal
            Symbol::Literal(literal) => {
                if to_overwrite.is_none() {
                    let object = propagate_error!(get_type_from_literal(&literal, memory_managers));
                    propagate_error!(object.static_assign_literal(memory_managers, &literal));
                    Ok(Some(object))
                }
                else {
                    propagate_error!(to_overwrite.unwrap().static_assign_literal(memory_managers, &literal));
                    Ok(None)
                }
            },
            // Recurse into arithmetic block
            Symbol::ArithmeticBlock(symbols) => {
                handle_arithmetic_section(memory_managers, reference_stack,
                                          symbols, to_overwrite,
                                          true)
            },
            _ => return Err("Only a name or literal can stand alone".to_string())
        }
    }
}
