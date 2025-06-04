use std::{iter::Peekable, str::Chars};

use crate::ast::{Instruction, Program, Value};

pub fn parse(input: &str) -> Program {
    let mut program = Program::new();
    let mut lines = input.lines();
    let mut line_number = 0;
    while let Some(line) = lines.next() {
        line_number += 1;
        let parsed_line = parse_line(line);
        match parsed_line {
            Ok(instruction) => program.add_instruction(instruction),
            Err(error) => eprintln!("[Line {}] {}", line_number, error),
        }
    }
    program
}

fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

fn parse_line(line: &str) -> Result<Instruction, String> {
    let mut chars = line.chars().peekable();
    // first parse the instruction name
    let instruction_name = parse_instruction_name(&mut chars)?;
    skip_whitespace(&mut chars);
    // then parse the arguments
    let args = parse_args(&mut chars)?;
    Ok(Instruction {
        instr: instruction_name,
        args,
    })
}

fn parse_instruction_name(chars: &mut Peekable<Chars>) -> Result<String, String> {
    let mut name = String::new();
    while let Some(c) = chars.peek() {
        if c.is_alphanumeric() {
            name.push(*c);
            chars.next();
        } else {
            break;
        }
    }
    if name.is_empty() {
        Err("Instruction name must be non-empty".to_string())
    } else {
        Ok(name)
    }
}

fn parse_args(chars: &mut Peekable<Chars>) -> Result<[Value; 3], String> {
    let mut args = [Value::Null, Value::Null, Value::Null];
    for i in 0..3 {
        match parse_arg(chars) {
            Some(Ok(arg)) => args[i] = arg,
            Some(Err(error)) => return Err(error),
            None => break,
        }
    }
    Ok(args)
}

fn parse_arg(chars: &mut Peekable<Chars>) -> Option<Result<Value, String>> {
    skip_whitespace(chars);
    if let Some(&c) = chars.peek() {
        return match c {
            '0'..='9' => Some(parse_number(chars)),
            c if c.is_alphabetic() => Some(parse_symbol(chars)),
            '*' => Some(parse_ptr(chars)),
            '\'' => Some(parse_char(chars)),
            _ => return Some(Err(format!("Unexpected character: {}", c))),
        };
    }
    None
}

fn parse_number(chars: &mut Peekable<Chars>) -> Result<Value, String> {
    let mut number = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) {
            number.push(c);
            chars.next();
        } else {
            break;
        }
    }
    if let Some('.') = chars.peek() {
        number.push('.');
        chars.next();
        while let Some(&c) = chars.peek() {
            if c.is_digit(10) {
                number.push(c);
                chars.next();
            } else {
                break;
            }
        }
    }
    if number.contains('.') {
        number
            .parse::<f64>()
            .map(Value::Float)
            .map_err(|e| format!("Invalid float: {}", e))
    } else {
        number
            .parse::<i64>()
            .map(Value::Integer)
            .map_err(|e| format!("Invalid integer: {}", e))
    }
}

fn parse_symbol(chars: &mut Peekable<Chars>) -> Result<Value, String> {
    let mut symbol = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            symbol.push(c);
            chars.next();
        } else {
            break;
        }
    }
    match symbol.as_str() {
        "true" => Ok(Value::Boolean(true)),
        "false" => Ok(Value::Boolean(false)),
        _ => Ok(Value::Symbol(symbol)),
    }
}

fn parse_ptr(chars: &mut Peekable<Chars>) -> Result<Value, String> {
    // skip the '*'
    chars.next();
    // then parse the uint
    let mut int_raw = String::new();
    while let Some(c) = chars.peek() {
        if c.is_digit(10) {
            int_raw.push(*c);
            chars.next();
        } else {
            break;
        }
    }
    if int_raw.is_empty() {
        return Err("Pointer must be followed by a number".to_string());
    } else if let Ok(int) = int_raw.parse::<u64>() {
        Ok(Value::Pointer(int))
    } else {
        return Err(format!("Invalid pointer: {}", int_raw));
    }
}

fn parse_char(chars: &mut Peekable<Chars>) -> Result<Value, String> {
    // skip the '
    chars.next();
    let char_val = chars
        .next()
        .ok_or("Char must be followed by a character".to_string())?;
    match chars.next() {
        Some('\'') => Ok(Value::Char(char_val)),
        Some(c) => Err(format!(
            "Char must be followed by a closing quote, got {}",
            c
        )),
        None => Err("Char must be followed by a closing quote".to_string()),
    }
}
