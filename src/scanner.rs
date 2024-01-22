use std::num::ParseFloatError;

use crate::token::Token;

fn match_number(i: &mut usize, chars: &Vec<char>, src: &String) -> Result<Token, ParseFloatError> {
    let start = *i;
    while chars[*i + 1].is_numeric() {
        *i += 1;
    }
    if chars[*i + 1] == '.' && chars[*i + 2].is_numeric() {
        *i += 1;
        while chars[*i + 1].is_numeric() {
            *i += 1;
        }
    }
    Ok(Token::Number(
        src.get(start..*i + 1)
            .expect("Index out of bounds")
            .parse::<f32>()?,
    ))
}

fn match_word(i: &mut usize, chars: &Vec<char>, src: &String) -> Result<Token, String> {
    let start = *i;
    while chars[*i + 1].is_ascii_alphanumeric() {
        *i += 1;
    }
    let word = src.get(start..*i + 1).expect("Index out of bounds");
    match word {
        "max" => Ok(Token::Max),
        "min" => Ok(Token::Min),
        "sqrt" => Ok(Token::Sqrt),
        "pow" => Ok(Token::Pow),
        "cos" => Ok(Token::Cos),
        "sin" => Ok(Token::Sin),
        "tan" => Ok(Token::Tan),
        "log" => Ok(Token::Log),
        _ => Err(format!("Unrecognized word {}", word)),
    }
}

pub fn tokenize(src: String) -> Result<Vec<Token>, String> {
    let mut i = 0;
    let chars: Vec<char> = src.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    while i < src.len() {
        let c = chars[i];
        match c {
            '*' => tokens.push(Token::Star),
            '-' => tokens.push(Token::Minus),
            '/' => tokens.push(Token::Slash),
            '+' => tokens.push(Token::Plus),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            ',' => tokens.push(Token::Comma),
            '%' => tokens.push(Token::Percent),
            ' ' => {}
            '\n' => {}
            _ => {
                if c.is_numeric() {
                    let num = match_number(&mut i, &chars, &src);
                    match num {
                        Ok(n) => tokens.push(n),
                        Err(e) => return Err(e.to_string()),
                    }
                } else if c.is_ascii_alphabetic() {
                    let word = match_word(&mut i, &chars, &src);
                    match word {
                        Ok(w) => tokens.push(w),
                        Err(e) => return Err(e.to_string()),
                    }
                } else {
                    return Err(format!("Invalid character {} at position {}", c, i));
                }
            }
        }
        i += 1;
    }
    Ok(tokens)
}
