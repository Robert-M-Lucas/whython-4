use crate::errors::create_line_error;
use crate::processing::symbols::{get_symbol, STRING_DELIMITERS, Symbol};
use crate::processing::symbols::Symbol::ArithmeticBlock;

pub fn get_symbols_from_line(line: &str) -> Result<Vec<Symbol>, String> {
    let mut symbol_line = Vec::new();

    let mut buffer = String::new();
    let mut in_string = false;
    let mut bracket_depth = 0;

    for c in line.chars() {
        if c == ' ' && bracket_depth == 0 && !in_string {
            if buffer.is_empty() { continue; }

            let symbol = get_symbol(&buffer);
            if symbol.is_none() {
                return Err(format!("Symbol '{}' not found", buffer));
            }
            symbol_line.push(symbol.unwrap());
            buffer.clear();
            continue;
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
            return create_line_error("Indentation must be a multiple of 4 spaces or single tabs".to_string(), line_index + 1);
        }
        let symbols = match get_symbols_from_line(&line[indentation_char_count..]) {
            Err(e) => return create_line_error(e, line_index),
            Ok(symbols) => symbols
        };
        output.push((indentation_count / 4, symbols));
    }

    Ok(output)
}