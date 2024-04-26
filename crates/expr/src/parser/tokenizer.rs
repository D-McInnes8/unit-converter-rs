use std::f64::consts::PI;

use log::{trace, warn};

use crate::{Function, Operator};

use super::error::ParseError;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Operator(Operator),
    Func(Function),
    Number(f64),
    Left,
    Right,
    Comma,
    Parameter(String),
}

pub fn parse(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut r = vec![];
    tokenizer(input, &mut r)?;
    Ok(r)
}

fn tokenizer(input: &str, result: &mut Vec<Token>) -> Result<(), ParseError> {
    trace!("Parsing expression slice \"{}\"", input);
    for (pos, c) in input.char_indices() {
        match c {
            '(' => {
                result.push(Token::Left);
                continue;
            }
            ')' => {
                result.push(Token::Right);
                continue;
            }
            _ => {}
        };

        let (token, new_pos) = match c {
            c if c.is_whitespace() => continue,
            c if c.is_operator() => operator(&input[pos..], result.last())?,
            c if c.is_numeric() => number(&input[pos..])?,
            c if c.is_alphabetic() => identifier(&input[pos..])?,
            _ => {
                warn!(
                    "Encountered unknown character '{}' while parsing expression \"{}\"",
                    c, input
                );
                continue;
            }
        };

        result.push(token);
        tokenizer(&input[pos + new_pos..], result)?;
        return Ok(());
    }

    Ok(())
}

fn number(input: &str) -> Result<(Token, usize), ParseError> {
    let mut end_pos: usize = input.len();

    for (pos, c) in input.char_indices() {
        if !c.is_numeric() && c != '.' && c != 'e' {
            end_pos = pos;
            break;
        }
    }

    let token = &input[0..end_pos];
    let num = match token.parse::<f64>() {
        Ok(n) => n,
        Err(err) => {
            return Err(ParseError::new(
                "Token is not a valid number",
                token,
                Some(err),
            ))
        }
    };

    Ok((Token::Number(num), end_pos))
}

fn identifier(input: &str) -> Result<(Token, usize), ParseError> {
    let mut end_pos: usize = input.len();

    for (pos, c) in input.char_indices() {
        if !c.is_alphabetic() {
            end_pos = pos;
            break;
        }
    }

    let token = &input[0..end_pos];
    if let Some(token) = func(token).or_else(|| param(token)) {
        Ok((token, end_pos))
    } else {
        Err(ParseError::new(
            "Identifier is not a valid function or parameter.",
            token,
            None as Option<ParseError>,
        ))
    }
}

fn func(input: &str) -> Option<Token> {
    match input.to_lowercase().as_str() {
        "max" => Some(Token::Func(Function::Max)),
        "min" => Some(Token::Func(Function::Min)),
        "sin" => Some(Token::Func(Function::Sin)),
        "cos" => Some(Token::Func(Function::Cos)),
        "tan" => Some(Token::Func(Function::Tan)),
        _ => None,
    }
}

fn param(input: &str) -> Option<Token> {
    Some(Token::Parameter(input.to_owned()))
}

fn operator(input: &str, prev: Option<&Token>) -> Result<(Token, usize), ParseError> {
    let mut end_pos: usize = input.len();
    let mut chars = input.char_indices().peekable();

    if let Some((pos, c)) = chars.next() {
        if c == '-' {
            if let Some((next_pos, _)) = chars.next_if(|&(_, n)| n == '>') {
                end_pos = next_pos + 1;
            } else {
                end_pos = pos + 1;
            }
        } else {
            end_pos = pos + 1;
            while !input.is_char_boundary(end_pos) {
                end_pos += 1;
            }
        }
    }

    let token = &input[0..end_pos];
    let operator = match token {
        "+" => Token::Operator(Operator::Addition),
        "-" | "−" => Token::Operator(Operator::Subtraction),
        "*" | "×" => Token::Operator(Operator::Multiplication),
        "/" | "÷" => Token::Operator(Operator::Division),
        "^" => Token::Operator(Operator::Exponentiation),
        "%" => Token::Operator(Operator::Modulus),
        "," => Token::Comma,
        "π" => Token::Number(PI),
        _ => {
            return Err(ParseError::new(
                "Token is not a valid operator",
                token,
                None as Option<ParseError>,
            ))
        }
    };

    // Convert unary operators into a special time so they're easier to evalute.
    if operator == Token::Operator(Operator::Subtraction) {
        match prev {
            None | Some(Token::Left) | Some(Token::Operator(_)) => {
                return Ok((Token::Operator(Operator::Negative), end_pos))
            }
            _ => (),
        }
    }

    Ok((operator, end_pos))
}

pub trait IsOperator {
    fn is_operator(&self) -> bool;
}

impl IsOperator for char {
    fn is_operator(&self) -> bool {
        matches!(
            self,
            '+' | '-' | '−' | '*' | '×' | '/' | '÷' | '^' | '%' | '<' | '>' | ',' | 'π'
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use std::vec;

    use log::debug;

    use super::*;

    #[test]
    fn operation() {
        let expected = vec![
            Token::Number(5.0),
            Token::Operator(Operator::Addition),
            Token::Number(10.0),
        ];
        let actual = parse("5 + 10");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn multibyte_unicode_character() {
        let expected = vec![
            Token::Number(5.0),
            Token::Operator(Operator::Division),
            Token::Operator(Operator::Negative),
            Token::Number(10.0),
        ];
        let actual = parse("5÷-10");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn function() {
        let expected = vec![
            Token::Func(Function::Sin),
            Token::Left,
            Token::Number(5.0),
            Token::Operator(Operator::Addition),
            Token::Number(10.0),
            Token::Right,
        ];
        let actual = parse("sin (5 + 10)");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn decimal() {
        let expected = vec![
            Token::Number(0.5),
            Token::Operator(Operator::Multiplication),
            Token::Number(1.04),
        ];
        let actual = parse("0.5 * 1.04");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn negative_number() {
        let expected = vec![
            Token::Operator(Operator::Negative),
            Token::Number(5.0),
            Token::Operator(Operator::Subtraction),
            Token::Operator(Operator::Negative),
            Token::Number(10.0),
        ];
        let actual = parse("-5 - -10");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn unary_expression_with_function() {
        let expected = vec![
            Token::Operator(Operator::Negative),
            Token::Func(Function::Max),
            Token::Left,
            Token::Number(5.3),
            Token::Comma,
            Token::Number(3.0),
            Token::Right,
        ];
        let actual = parse("-max(5.3, 3)");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn nested() {
        let expected = vec![
            Token::Func(Function::Sin),
            Token::Left,
            Token::Left,
            Token::Number(1.5),
            Token::Operator(Operator::Addition),
            Token::Number(5.0),
            Token::Right,
            Token::Operator(Operator::Multiplication),
            Token::Number(2.0),
            Token::Right,
            Token::Operator(Operator::Subtraction),
            Token::Number(76.0),
            Token::Operator(Operator::Multiplication),
            Token::Left,
            Token::Func(Function::Sin),
            Token::Left,
            Token::Number(5.0),
            Token::Right,
            Token::Operator(Operator::Addition),
            Token::Number(5.0),
            Token::Right,
        ];
        let actual = parse("sin ((1.5 + 5) * 2) - 76 * (sin(5) + 5)");
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
        let actual = parse("sin ( max ( 2, 3 ) ÷ 3 × π )");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn parameter_single_character() {
        let input = "a + 5";
        let expected = vec![
            Token::Parameter(String::from("a")),
            Token::Operator(Operator::Addition),
            Token::Number(5.0),
        ];
        let actual = parse(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parameter_multiple_characters() {
        let input = "param + 5";
        let expected = vec![
            Token::Parameter(String::from("param")),
            Token::Operator(Operator::Addition),
            Token::Number(5.0),
        ];
        let actual = parse(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn multiple_parameters() {
        let input = "a+b";
        let expected = vec![
            Token::Parameter(String::from("a")),
            Token::Operator(Operator::Addition),
            Token::Parameter(String::from("b")),
        ];
        let actual = parse(input).unwrap();
        assert_eq!(expected, actual);
    }
}
