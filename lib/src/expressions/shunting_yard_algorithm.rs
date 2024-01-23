use std::collections::VecDeque;
use std::rc::Rc;

use log::debug;

use super::error::ParseError;
use super::expression::{OperationType, Operator};
use super::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub struct AbstractSyntaxTree {
    val: OperationType,
    left: Option<Rc<AbstractSyntaxTree>>,
    right: Option<Rc<AbstractSyntaxTree>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
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

pub fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, ParseError> {
    let mut output = Vec::with_capacity(tokens.len());
    let mut stack: Vec<Token> = Vec::with_capacity(tokens.len());

    for token in tokens {
        match token {
            Token::Number { value } => output.push(token),
            Token::Operator { value } => {
                let o1 = value;
                loop {
                    if let Some(o2) = stack.last() {
                        match *o2 {
                            Token::Left => {
                                stack.push(token);
                                break;
                            }
                            Token::Operator { value }
                                if (value.prec() > o1.prec())
                                    || (value.prec() == o1.prec()
                                        && o1.assoc() == Associativity::Left) =>
                            {
                                //let a = stack.pop();
                                output.push(stack.pop().unwrap());
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        stack.push(token);
                        break;
                    }
                }
            }
            Token::Func { value } => {}
            Token::Left => stack.push(token),
            Token::Right => loop {
                if let Some(top) = stack.last() {
                    if *top == Token::Left {
                        _ = stack.pop();
                        break;
                    }

                    output.push(stack.pop().unwrap());
                }
                let err: Option<ParseError> = None;
                return Err(ParseError::new("Mismatched parentheses", "", err));
            },
        };
    }

    println!("{:?}", stack);

    while let Some(operator) = stack.pop() {
        if operator == Token::Left || operator == Token::Right {
            let err: Option<ParseError> = None;
            return Err(ParseError::new("", "", err));
        }
        output.push(operator);
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shunting_yard_test() {
        let tokens: Vec<Token> = vec![
            Token::Number { value: 5.0 },
            Token::Operator {
                value: Operator::Addition,
            },
            Token::Number { value: 10.0 },
        ];
        let actual = shunting_yard(tokens);
        let expected: Vec<Token> = vec![];
        assert_eq!(expected, actual.unwrap());
    }

    /*#[test]
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
    }*/
}
