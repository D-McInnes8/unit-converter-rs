use std::f64::consts::PI;

use super::error::ParseError;
use super::expression::{Function, Operator};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Operator(Operator),
    Func(Function),
    Number(f64),
    Left,
    Right,
    Comma,
}

#[macro_export]
macro_rules! token_operator {
    ($e:expr) => {
        Token::Operator($e)
    };
}

#[macro_export]
macro_rules! token_func {
    ($e:expr) => {
        Token::Func($e)
    };
}

#[macro_export]
macro_rules! token_number {
    ($e:expr) => {
        Token::Number($e)
    };
}

pub fn get_tokens(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut results = vec![];
    parse(input, &mut results)?;
    Ok(results)
}

fn parse<'a>(input: &str, tokens: &'a mut Vec<Token>) -> Result<(), ParseError> {
    for (pos, c) in input.char_indices() {
        match c {
            c if c.is_whitespace() => {}
            c if c == '+' => tokens.push(Token::Operator(Operator::Addition)),
            c if c == '-' => tokens.push(Token::Operator(Operator::Subtraction)),
            c if c == '*' || c == '×' => tokens.push(Token::Operator(Operator::Multiplication)),
            c if c == '/' || c == '÷' => tokens.push(Token::Operator(Operator::Division)),
            c if c == '^' => tokens.push(Token::Operator(Operator::Exponentiation)),
            c if c == '%' => tokens.push(Token::Operator(Operator::Modulus)),
            c if c == '(' => tokens.push(Token::Left),
            c if c == ')' => tokens.push(Token::Right),
            c if c == ',' => tokens.push(Token::Comma),
            c if c == 'π' => tokens.push(Token::Number(PI)),
            c if c.is_numeric() => {
                let (remaining, number) = parse_number(&input[pos..])?;
                tokens.push(number);
                parse(remaining, tokens)?;
                return Ok(());
            }
            c if c.is_alphabetic() => {
                let (remaining, func) = parse_func(&input[pos..])?;
                tokens.push(func);
                parse(remaining, tokens)?;
                return Ok(());
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_number(input: &str) -> Result<(&str, Token), ParseError> {
    let mut buffer: Vec<char> = vec![];
    let mut end_pos: usize = input.len();

    for (pos, c) in input.char_indices() {
        if !c.is_numeric() && c != '.' {
            end_pos = pos;
            break;
        }

        buffer.push(c);
    }

    let token: String = buffer.into_iter().collect();
    let num = match token.parse::<f64>() {
        Ok(n) => n,
        Err(err) => return Err(ParseError::new("Not a valid number", &token, Some(err))),
    };

    Ok((&input[end_pos..input.len()], Token::Number(num)))
}

fn parse_func(input: &str) -> Result<(&str, Token), ParseError> {
    let mut buffer: Vec<char> = vec![];
    let mut end_pos: usize = input.len();

    for (pos, c) in input.char_indices() {
        if !c.is_alphabetic() {
            end_pos = pos;
            break;
        }

        buffer.push(c);
    }

    let token: String = buffer.into_iter().collect();
    let function = match token.to_lowercase().as_str() {
        "max" => Function::Max,
        "min" => Function::Min,
        "sin" => Function::Sin,
        "cos" => Function::Cos,
        "tan" => Function::Tan,
        _ => {
            return Err(ParseError::new(
                "Not a valid function name",
                &token,
                None as Option<ParseError>,
            ));
        }
    };

    Ok((&input[end_pos..input.len()], Token::Func(function)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation() {
        let expected = vec![
            token_number!(5.0),
            token_operator!(Operator::Addition),
            token_number!(10.0),
        ];
        let actual = get_tokens("5 + 10");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn function() {
        let expected = vec![
            token_func!(Function::Sin),
            Token::Left,
            token_number!(5.0),
            token_operator!(Operator::Addition),
            token_number!(10.0),
            Token::Right,
        ];
        let actual = get_tokens("sin (5 + 10)");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn decimal() {
        let expected = vec![
            token_number!(0.5),
            token_operator!(Operator::Multiplication),
            token_number!(1.04),
        ];
        let actual = get_tokens("0.5 * 1.04");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn negative_number() {
        let expected = vec![
            token_operator!(Operator::Subtraction),
            token_number!(5.0),
            token_operator!(Operator::Subtraction),
            token_operator!(Operator::Subtraction),
            token_number!(10.0),
        ];
        let actual = get_tokens("-5 - -10");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn nested() {
        let expected = vec![
            token_func!(Function::Sin),
            Token::Left,
            Token::Left,
            token_number!(1.5),
            token_operator!(Operator::Addition),
            token_number!(5.0),
            Token::Right,
            token_operator!(Operator::Multiplication),
            token_number!(2.0),
            Token::Right,
            token_operator!(Operator::Subtraction),
            token_number!(76.0),
            token_operator!(Operator::Multiplication),
            Token::Left,
            token_func!(Function::Sin),
            Token::Left,
            token_number!(5.0),
            Token::Right,
            token_operator!(Operator::Addition),
            token_number!(5.0),
            Token::Right,
        ];
        let actual = get_tokens("sin ((1.5 + 5) * 2) - 76 * (sin(5) + 5)");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn nested_functions() {
        let expected = vec![
            Token::Func(Function::Sin),
            Token::Left,
            Token::Func(Function::Max),
            Token::Left,
            Token::Number(2.0),
            Token::Comma,
            Token::Number(3.0),
            Token::Right,
            Token::Operator(Operator::Division),
            Token::Number(3.0),
            Token::Operator(Operator::Multiplication),
            Token::Number(PI),
            Token::Right,
        ];
        let actual = get_tokens("sin ( max ( 2, 3 ) ÷ 3 × π )");
        assert_eq!(expected, actual.unwrap());
    }
}
