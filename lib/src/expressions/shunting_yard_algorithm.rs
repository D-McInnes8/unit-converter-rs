use std::collections::VecDeque;
use std::rc::Rc;

use log::debug;

#[derive(Debug, PartialEq)]
pub enum OperationType {
    Number { value: f64 },
    BinaryExpression { operator: Operator },
    Function { name: String, value: f64 },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq)]
pub struct AbstractSyntaxTree {
    val: OperationType,
    left: Option<Rc<AbstractSyntaxTree>>,
    right: Option<Rc<AbstractSyntaxTree>>,
}
pub fn parse2(input: &str) -> Option<AbstractSyntaxTree> {
    None
}

#[derive(Debug, PartialEq)]
pub struct AstNode {
    val: OperationType,
    left: Option<Box<AstNode>>,
    right: Option<Box<AstNode>>,
}

pub fn parse(input: &str) -> Option<AstNode> {
    None
}

fn shunting_yard(input: &str) {
    let mut output: VecDeque<i32> = VecDeque::new();
    let mut operator: Vec<i32> = vec![];
}

pub fn get_tokens(input: &str) -> Vec<String> {
    let mut results = vec![];
    consume(input, &mut results);
    results
}

fn consume<'a>(input: &str, tokens: &'a mut Vec<String>) {
    for (pos, c) in input.char_indices() {
        match c {
            c if c.is_whitespace() => {}
            c if c.is_numeric() => {
                debug!("Parsing numeric");
                let (remaining, number) = consume_number(&input[pos..]);
                tokens.push(number);
                debug!("Remaining work: {}", remaining);
                consume(remaining, tokens);
                return;
            }
            c if c == '+'
                || c == '-'
                || c == '*'
                || c == '/'
                || c == '^'
                || c == '('
                || c == ')' =>
            {
                tokens.push(c.to_string())
            }
            c if c.is_alphabetic() => {
                let (remaining, func) = consume_func(&input[pos..]);
                tokens.push(func);
                consume(remaining, tokens);
                return;
            }
            _ => {}
        }
    }
}

fn consume_number(input: &str) -> (&str, String) {
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
    //let num = token.parse::<f64>().unwrap();

    (&input[end_pos..input.len()], token)
}

fn consume_func(input: &str) -> (&str, String) {
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
    (&input[end_pos..input.len()], token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_operations() {
        let expected = vec!["5", "+", "10"];
        let actual = get_tokens("5 + 10");
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizer_functions() {
        let expected = vec!["sin", "(", "5", "+", "10", ")"];
        let actual = get_tokens("sin (5 + 10)");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_binary_expression() {
        let expected = Some(AstNode {
            val: OperationType::BinaryExpression {
                operator: Operator::Addition,
            },
            left: Some(Box::new(AstNode {
                val: OperationType::Number { value: 5.0 },
                left: None,
                right: None,
            })),
            right: Some(Box::new(AstNode {
                val: OperationType::Number { value: 10.0 },
                left: None,
                right: None,
            })),
        });
        let actual = parse("5 + 10");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_binary_expression2() {
        let expected = Some(AbstractSyntaxTree {
            val: OperationType::BinaryExpression {
                operator: Operator::Addition,
            },
            left: Some(Rc::new(AbstractSyntaxTree {
                val: OperationType::Number { value: 5.0 },
                left: None,
                right: None,
            })),
            right: Some(Rc::new(AbstractSyntaxTree {
                val: OperationType::Number { value: 10.0 },
                left: None,
                right: None,
            })),
        });
        let actual = parse2("5 + 10");

        assert_eq!(expected, actual);
    }
}
