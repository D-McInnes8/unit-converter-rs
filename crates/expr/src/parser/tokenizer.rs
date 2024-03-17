use std::f64::consts::PI;

use log::{debug, warn};

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
    Unit(String),
    Parameter(String),
}

pub fn parse(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut r = vec![];
    tokenizer(input, &mut r)?;
    Ok(r)
}

fn tokenizer(input: &str, result: &mut Vec<Token>) -> Result<(), ParseError> {
    debug!("Parsing expression slice \"{}\"", input);
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
            '{' => param(&input[pos..])?,
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
    if let Some(token) = func(token).or_else(|| unit(token)) {
        return Ok((token, end_pos));
    } else {
        return Err(ParseError::new(
            "Identifier is not a valid function or unit of measurement",
            token,
            None as Option<ParseError>,
        ));
    }
}

fn func(input: &str) -> Option<Token> {
    match input.to_lowercase().as_str() {
        "max" => Some(Token::Func(Function::Max)),
        "min" => Some(Token::Func(Function::Min)),
        "sin" => Some(Token::Func(Function::Sin)),
        "cos" => Some(Token::Func(Function::Cos)),
        "tan" => Some(Token::Func(Function::Tan)),
        "to" => Some(Token::Operator(Operator::Conversion)),
        _ => None,
    }
}

fn unit(input: &str) -> Option<Token> {
    Some(Token::Unit(input.to_owned()))
}

fn param(input: &str) -> Result<(Token, usize), ParseError> {
    let mut end_pos: usize = input.len();

    for (pos, c) in input.char_indices() {
        if c == '}' {
            end_pos = pos;
            break;
        }
    }

    let token = &input[1..end_pos];
    Ok((Token::Parameter(token.to_owned()), end_pos))
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
        "->" => Token::Operator(Operator::Conversion),
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
        match self {
            '+' | '-' | '−' | '*' | '×' | '/' | '÷' | '^' | '%' | '<' | '>' | ',' | 'π' => {
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

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
    fn valid_temperature_conversion() {
        let input = "20C -> F";

        let expected = vec![
            Token::Number(20.0),
            Token::Unit(String::from("C")),
            Token::Operator(Operator::Conversion),
            Token::Unit(String::from("F")),
        ];
        let actual = parse(input).unwrap();
        assert_eq!(expected, actual);
    }

    /*#[test]
    fn conversion_invalid_unit() {
        let input = "20x -> F";

        let actual = parse(input);
        assert!(actual.is_err());
    }*/

    #[test]
    fn conversion_same_characters_different_case() {
        let input = "1Mm -> mm";

        let expected = vec![
            Token::Number(1.0),
            Token::Unit(String::from("Mm")),
            Token::Operator(Operator::Conversion),
            Token::Unit(String::from("mm")),
        ];
        let actual = parse(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn conversion_e_notation() {
        let input = "1.079913e9km -> nmi";

        let expected = vec![
            Token::Number(1079913000.0),
            Token::Unit(String::from("km")),
            Token::Operator(Operator::Conversion),
            Token::Unit(String::from("nmi")),
        ];
        let actual = parse(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn combined_conversion_and_expression() {
        let input = "5km + 5 -> m + 10";
        let expected = vec![
            Token::Number(5.0),
            Token::Unit(String::from("km")),
            Token::Operator(Operator::Addition),
            Token::Number(5.0),
            Token::Operator(Operator::Conversion),
            Token::Unit(String::from("m")),
            Token::Operator(Operator::Addition),
            Token::Number(10.0),
        ];
        let actual = parse(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parameter() {
        let input = "{a} + 5";
        let expected = vec![
            Token::Parameter(String::from("a")),
            Token::Operator(Operator::Addition),
            Token::Number(5.0),
        ];
        let actual = parse(input).unwrap();
        assert_eq!(expected, actual);
    }
}
