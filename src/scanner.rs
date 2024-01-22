use std::num::ParseFloatError;

use crate::token::Token;

fn match_number(i: &mut usize, chars: &Vec<char>, src: &String) -> Result<Token, ParseFloatError> {
    let start = *i;
    while *i + 1 < src.len() && chars[*i + 1].is_numeric() {
        *i += 1;
    }
    if *i + 1 < src.len() && chars[*i + 1] == '.' {
        if *i + 2 < src.len() && chars[*i + 2].is_numeric() {
            *i += 1;
            while chars[*i + 1].is_numeric() {
                *i += 1;
            }
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
    while *i + 3 < src.len() && chars[*i + 1].is_ascii_alphanumeric()
        || chars[*i + 1] == '_'
        || chars[*i + 1] == '-'
    {
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

#[cfg(test)]
mod tests {
    use crate::token::Token;

    #[test]
    fn allowed_strings() {
        let testcases = [
            (
                "1 + 1",
                vec![Token::Number(1.0), Token::Plus, Token::Number(1.0)],
            ),
            (
                "1 - 2 * 3 / 4",
                vec![
                    Token::Number(1.0),
                    Token::Minus,
                    Token::Number(2.0),
                    Token::Star,
                    Token::Number(3.0),
                    Token::Slash,
                    Token::Number(4.0),
                ],
            ),
            (
                "sin(cos(tan(log(2.0))))",
                vec![
                    Token::Sin,
                    Token::OpenParen,
                    Token::Cos,
                    Token::OpenParen,
                    Token::Tan,
                    Token::OpenParen,
                    Token::Log,
                    Token::OpenParen,
                    Token::Number(2.0),
                    Token::CloseParen,
                    Token::CloseParen,
                    Token::CloseParen,
                    Token::CloseParen,
                ],
            ),
            (
                "max(1.0,2.0,3.0,min(4.0,5.0,6.0))",
                vec![
                    Token::Max,
                    Token::OpenParen,
                    Token::Number(1.0),
                    Token::Comma,
                    Token::Number(2.0),
                    Token::Comma,
                    Token::Number(3.0),
                    Token::Comma,
                    Token::Min,
                    Token::OpenParen,
                    Token::Number(4.0),
                    Token::Comma,
                    Token::Number(5.0),
                    Token::Comma,
                    Token::Number(6.0),
                    Token::CloseParen,
                    Token::CloseParen,
                ],
            ),
        ];
        for case in testcases.iter() {
            let res = super::tokenize(case.0.to_string());
            assert!(res.is_ok());
            assert_eq!(
                res.clone().unwrap().len(),
                case.1.len(),
                "\n  left: {:?} \n right: {:?}",
                res.clone().unwrap(),
                case.1
            );
            assert_eq!(res.unwrap(), case.1);
        }
    }

    #[test]
    fn disallowed_strings() {
        let testcases = [
            "@", "!", "#", "$", "|", "{", "}", ":", "\"", "\'", ";", ">", "<", "`", "~", "max-",
        ];
        for case in testcases.iter() {
            let res = super::tokenize(case.to_string());
            assert!(res.is_err(), "{:?}", res.unwrap());
        }
    }
}
