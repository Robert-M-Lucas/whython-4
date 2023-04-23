use debugless_unwrap::DebuglessUnwrapErr;
use crate::errors::create_line_error;
use crate::processing::symbols::{get_symbol, STRING_DELIMITERS, Symbol};
use crate::processing::symbols::Symbol::{ArithmeticBlock};

pub fn get_symbols_from_line(line: &str) -> Result<Vec<Symbol>, String> {
    let mut symbol_line = Vec::new();

    let mut buffer = String::new();
    let mut in_string = false;
    let mut bracket_depth = 0;
    let mut in_indexer = false;
    let mut indexing_start: usize = 0;

    fn process_buffer(buffer: &mut String, symbol_line: &mut Vec<Symbol>) -> Result<(), String> {
        if buffer.is_empty() {
            return Ok(())
        }

        let symbol = get_symbol(&buffer);
        if symbol.is_none() {
            return Err(format!("Symbol '{}' not found", buffer));
        }
        symbol_line.push(symbol.unwrap());
        buffer.clear();
        Ok(())
    }

    for c in line.chars() {
        if c == '#' && !in_string {
            break;
        }

        if bracket_depth == 0 && !in_string {
            //? Process buffer and ignore c
            match
                match c {
                    ' ' => Some(process_buffer(&mut buffer, &mut symbol_line)),
                    _ => None
                }
            {
                Some(value) => match value {
                    Err(e) => return Err(e),
                    Ok(_) => continue
                },
                None => {}
            }

            //? Process buffer and then treat c normally
            match
                match c {
                    '(' => Some(process_buffer(&mut buffer, &mut symbol_line)),
                    _ => None
                }
            {
                Some(value) => match value {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                },
                None => {}
            }

            //? If buffer is empty, process character alone
            match
                match c {
                    '!' => {
                        if buffer.len() != 0 { None }
                        else {
                            buffer.push(c);
                            Some(process_buffer(&mut buffer, &mut symbol_line))
                        }
                    },
                    _ => None
                }
            {
                Some(value) => match value {
                    Err(e) => return Err(e),
                    Ok(_) => continue
                },
                None => {}
            }

            //? Process character alone
            match
                match c {
                    ',' => {
                        let r = process_buffer(&mut buffer, &mut symbol_line);
                        if r.is_err() { return Err(r.debugless_unwrap_err()); }
                        buffer.push(c);
                        Some(process_buffer(&mut buffer, &mut symbol_line))
                    },
                    _ => None
                }
            {
                Some(value) => match value {
                    Err(e) => return Err(e),
                    Ok(_) => continue
                },
                None => {}
            }
        }

        if c == ')' && !in_string {
            bracket_depth -= 1;

            if bracket_depth == 0 {
                symbol_line.push(
                    match get_symbols_from_line(buffer.as_str()) {
                        Ok(symbols) => ArithmeticBlock(symbols),
                        Err(e) => return Err(e)
                    }
                );
                buffer.clear();

            }
            else if bracket_depth < 0 {
                return Err("Closing bracket found with no corresponding opening bracket".to_string())
            }
            else {
                buffer.push(c);
            }

            continue;
        }

        if STRING_DELIMITERS.contains(&c) {
            in_string = !in_string;
        }

        if c == '(' && !in_string {
            if bracket_depth != 0 {
                buffer.push(c);
            }
            bracket_depth += 1;
            continue;
        }

        if c == ']' && !in_string {
            if !buffer.is_empty() {
                match process_buffer(&mut buffer, &mut symbol_line) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                };
            }
            if !in_indexer {
                return Err("Closing indexer bracket found with no corresponding opening bracket".to_string());
            }
            if symbol_line.len() - indexing_start > 1 {
                return Err("Indexers may only contain one symbol".to_string());
            }
            if symbol_line.len() - indexing_start < 1 {
                return Err("Indexer must contain a symbol".to_string());
            }
            let symbol = symbol_line.pop().unwrap();
            symbol_line.push(Symbol::Indexer(Box::new(symbol)));
            in_indexer = false;
            continue;
        }

        if c == '[' && !in_string && in_indexer {
            if !buffer.is_empty() {
                match process_buffer(&mut buffer, &mut symbol_line) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                };
            }
            if in_indexer {
                return Err("Recursive indexing not permitted".to_string());
            }
            indexing_start = symbol_line.len();
            in_indexer = true;
            continue;
        }

        buffer.push(c);
    }

    if in_string {
        return Err("Unclosed string".to_string())
    }

    if bracket_depth != 0 {
        return Err("Unclosed brackets".to_string())
    }

    if !buffer.is_empty() {
        let symbol = get_symbol(&buffer);
        if symbol.is_none() { return Err(format!("Symbol '{}' not found", buffer)); }
        symbol_line.push(symbol.unwrap());
    }

    Ok(symbol_line)
}

pub fn convert_to_symbols(data: String) -> Result<Vec<(usize, Vec<Symbol>)>, String> {
    let mut output = Vec::new();


    for (line_index, line) in  data.lines().enumerate() {
        let mut indentation_count: usize = 0;
        let mut indentation_char_count: usize = 0;
        for c in line.chars() {
            if c == ' ' { indentation_count += 1 }
            else if c == '\t' { indentation_count += 4 }
            else { break }
            indentation_char_count += 1;
        }
        if indentation_count % 4 != 0 {
            return create_line_error("Indentation must be a multiple of 4 spaces or single tabs"
                                         .to_string(), line_index + 1);
        }
        let symbols = match get_symbols_from_line(&line[indentation_char_count..]) {
            Err(e) => return create_line_error(e, line_index),
            Ok(symbols) => symbols
        };
        output.push((indentation_count / 4, symbols));
    }

    Ok(output)
}