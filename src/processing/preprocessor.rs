use crate::error::create_error;
use crate::processing::symbols::{get_symbol, STRING_DELIMITERS, Symbol};

pub fn get_symbols_from_line(line: &str) -> Result<Vec<Symbol>, String> {
    todo!()
}

pub fn convert_to_symbols(data: String) -> Result<Vec<(usize, Vec<Symbol>)>, String> {
    let mut output = Vec::new();
    let mut line = Vec::new();

    let mut buffer = String::new();
    let mut in_string = false;
    let mut in_brackets = false;

    let mut indentation_count: i32 = 0;
    let mut current_indentation: usize = 0;


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
            return create_error("Indentation must be a multiple of 4 spaces or single tabs".to_string(), line_index + 1);
        }
        let symbols = get_symbols_from_line(&line[indentation_char_count..]);
        if symbols.is_err() { return create_error(symbols.unwrap_err()); }
    }

    for c in data.chars() {
        if indentation_count != -1 {
            if c == ' ' {
                indentation_count += 1;
                continue;
            }
            else {
                if indentation_count % 4 != 0 {
                    return create_error("Indentation must be a multiple of 4 spaces or single tabs".to_string(), output.len() + 1);
                }
                current_indentation = (indentation_count / 4) as usize;
                indentation_count = -1;
            }
        }

        if c == '\n' || c == '\r' || c == ' ' {
            if c == '\n' && in_string { return create_error("Strings cannot span multiple lines".to_string(), output.len() + 1); }
            if buffer.is_empty() {
                if c == '\n' {
                    output.push((current_indentation, line));
                    line = Vec::new();
                }
                continue;
            }

            let symbol = get_symbol(&buffer);
            if symbol.is_none() { return create_error(format!("Symbol '{}' not found", buffer), output.len() + 1); }
            line.push(symbol.unwrap());
            buffer.clear();

            if c == '\n' {
                output.push((current_indentation, line));
                line = Vec::new();
            }

            continue;
        }

        if STRING_DELIMITERS.contains(&c) {
            in_string = !in_string;
        }

        buffer.push(c);
    }

    if !buffer.is_empty() {
        let symbol = get_symbol(&buffer);
        if symbol.is_none() { return create_error(format!("Symbol '{}' not found", buffer), output.len() + 1); }
        line.push(symbol.unwrap());
    }

    output.push((current_indentation, line));

    Ok(output)
}