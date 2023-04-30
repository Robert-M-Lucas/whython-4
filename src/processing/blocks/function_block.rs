use std::mem::size_of;
use crate::processing::blocks::BlockHandler;
use crate::processing::instructions::jump_instruction_3::JumpInstruction;
use crate::processing::instructions::jump_variable_instruction_4::JumpVariableInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Literal, Symbol, try_arithmetic_block_into_parameters};
use crate::processing::types::{get_type, Type};
use crate::processing::types::function::FunctionType;
use crate::propagate_error;
use crate::util::warn;

pub struct FunctionBlock {
    jump_variable: Option<usize>,
    skip_instruction: Option<JumpInstruction>,
}

impl FunctionBlock {
    pub fn new() -> Box<dyn BlockHandler> {
        Box::new(
            Self {
                jump_variable: None,
                skip_instruction: None
            }
        )
    }
}

impl BlockHandler for FunctionBlock {
    fn on_entry(&mut self, memory_managers: &mut MemoryManagers, reference_stack: &mut ReferenceStack, symbol_line: &Vec<Symbol>) -> Result<(), String> {
        warn("Functions currently don't support recursion (including indirect)");

        fn formatting_error() -> String {
            "Function declaration must be formatted 'fn [FUNCTION NAME] ([PARAMETER LIST]) {OPTIONAL [RETURN TYPE] [DEFAULT RETURN VALUE]}'"
                .to_string()
        }

        //? Insert skip instruction

        self.skip_instruction = Some(JumpInstruction::new_alloc(memory_managers, 0));

        //? Extract name and parameters
        if symbol_line.len() != 3 && symbol_line.len() != 5 {
            return Err(formatting_error());
        }

        let name = match &symbol_line[1] {
            Symbol::Name(name) => name.clone(),
            _ => return Err(formatting_error()),
        };

        let parameters = match &symbol_line[2] {
            Symbol::ArithmeticBlock(_) => {
                match propagate_error!(try_arithmetic_block_into_parameters(&symbol_line[2])) {
                    Literal::ParameterList(list) => list,
                    _ => panic!()
                }
            },
            _ => return Err(formatting_error()),
        };
        
        let return_type;
        if symbol_line.len() == 3 {
            return_type = None;
        }
        else {
            return_type = match &symbol_line[3] {
                Symbol::Type(type_symbol) => {
                    Some(
                        propagate_error!(get_type(type_symbol, memory_managers))
                    )
                },
                _ => return Err(formatting_error()),
            };

            let default_return_value = match &symbol_line[4] {
                Symbol::Literal(literal) => literal,
                _ => return Err(formatting_error())
            };

            propagate_error!(return_type.as_ref().unwrap().static_assign_literal(memory_managers, default_return_value));
        }
        
        //? Register and clone parameters
        let mut to_assign = Vec::new();
        
        for (type_symbol, name) in parameters {
            let created_type = propagate_error!(get_type(&type_symbol, memory_managers));
            to_assign.push(created_type.clone());
            propagate_error!(reference_stack.register_variable(created_type, name));
        }

        if return_type.is_some() {
            propagate_error!(reference_stack.register_variable(return_type.as_ref().unwrap().clone(), "return".to_string()));
        }

        self.jump_variable = Some(memory_managers.variable_memory.reserve(size_of::<usize>()));

        //? Register function in above handler
        let function = 
            FunctionType::create_empty(to_assign,
                                       return_type,
                                       memory_managers.program_memory.get_position(),
                                        self.jump_variable.unwrap()
            );



        propagate_error!(reference_stack.register_variable_with_offset(
            Type::new(Box::new(function), memory_managers), name, 1));

        Ok(())
    }

    fn on_forced_exit(&mut self, memory_managers: &mut MemoryManagers, _reference_stack: &mut ReferenceStack) -> Result<(), String> {
        JumpVariableInstruction::new_alloc(memory_managers, self.jump_variable.unwrap());

        self.skip_instruction.as_mut().unwrap().set_destination(memory_managers, memory_managers.program_memory.get_position());

        Ok(())
    }
}