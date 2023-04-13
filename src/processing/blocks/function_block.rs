use crate::processing::blocks::BlockHandler;
use crate::processing::instructions::jump_if_instruction_2::JumpIfInstruction;
use crate::processing::lines::arithmetic::handle_arithmetic_section;
use crate::processing::processor::MemoryManagers;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Literal, Symbol, try_arithmetic_block_into_parameters};
use crate::processing::types::{get_type, Type, TypeSymbol};
use crate::processing::types::function::FunctionType;

pub struct FunctionBlock {
    jump_instruction: Option<JumpIfInstruction>
}

impl FunctionBlock {
    pub fn new() -> Box<dyn BlockHandler> {
        Box::new(
            Self {
                jump_instruction: None
            }
        )
    }
}

impl BlockHandler for FunctionBlock {
    fn on_entry(&mut self, memory_managers: &mut MemoryManagers, reference_stack: &mut ReferenceStack, symbol_line: &Vec<Symbol>) -> Result<(), String> {
        fn formatting_error() -> String {
            "Function declaration must be formatted 'fn [FUNCTION NAME] ([PARAMETER LIST]) {OPTIONAL [RETURN TYPE] [DEFAULT RETURN VALUE]}'"
                .to_string()
        }

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
                match try_arithmetic_block_into_parameters(&symbol_line[2]) {
                    Err(e) => return Err(e),
                    Ok(value) => {
                        match value {
                            Literal::ParameterList(list) => list,
                            _ => panic!()
                        }
                    }
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
                        match get_type(type_symbol, memory_managers) {
                            Err(e) => return Err(e),
                            Ok(value) => value
                        }
                    )
                },
                _ => return Err(formatting_error()),
            };

            let default_return_value = match &symbol_line[4] {
                Symbol::Literal(literal) => literal,
                _ => return Err(formatting_error())
            };

            match return_type.as_ref().unwrap().static_assign_literal(memory_managers, default_return_value) {
                Err(e) => return Err(e),
                Ok(_) => {}
            };
        }
        
        //? Register and clone parameters
        let mut to_assign = Vec::new();
        
        for (type_symbol, name) in parameters {
            let created_type = match get_type(&type_symbol, memory_managers) {
                Err(e) => return Err(e),
                Ok(value) => value
            };
            to_assign.push(created_type.clone());
            match reference_stack.register_variable(created_type, name) {
                Err(e) => return Err(e),
                _ => {},
            };
        }

        if return_type.is_some() {
            match reference_stack.register_variable(return_type.as_ref().unwrap().clone(), "return".to_string()) {
                Err(e) => return Err(e),
                _ => {},
            };
        }
        
        //? Register function in above handler
        let function = 
            FunctionType::create_empty(to_assign,
                                       return_type,
                                       memory_managers.program_memory.get_position());
        
        match reference_stack.register_variable_with_offset(
            Type::new(Box::new(function), memory_managers), name, 1) {
            Err(e) => return Err(e),
            _ => {}
        };

        Ok(())
    }

    fn on_forced_exit(&mut self, _memory_managers: &mut MemoryManagers, _reference_stack: &mut ReferenceStack, _symbol_line: &Vec<Symbol>) -> Result<(), String> {
        //? Remove local parameter handler

        //? Save local parameters in function type

        //? Add function type to outer reference stack

        Ok(())
    }
}