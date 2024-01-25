use std::rc::Rc;

use super::error::{ExpressionError, ParseError};
use super::expression::OperationType;
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

pub fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, ExpressionError> {
    let mut output = Vec::with_capacity(tokens.len());
    let mut stack: Vec<Token> = Vec::with_capacity(tokens.len());

    for token in tokens {
        match token {
            Token::Number(_) => {
                output.push(token);
            }
            Token::Func(_) => {
                stack.push(token);
            }
            Token::Operator(o1) => {
                loop {
                    if let Some(o2) = stack.last() {
                        match *o2 {
                            Token::Operator(o2)
                                if (o2.prec() > o1.prec())
                                    || (o2.prec() == o1.prec()
                                        && o1.assoc() == Associativity::Left) =>
                            {
                                output.push(stack.pop().unwrap());
                            }
                            _ => {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                stack.push(token);
            }
            Token::Comma => {
                while let Some(top) = stack.last() {
                    if *top != Token::Left {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
            }
            Token::Left => {
                stack.push(token);
            }
            Token::Right => loop {
                if let Some(top) = stack.last() {
                    if *top == Token::Left {
                        _ = stack.pop();

                        if let Some(Token::Func(_)) = stack.last() {
                            output.push(stack.pop().unwrap());
                        }

                        break;
                    }

                    output.push(stack.pop().unwrap());
                } else {
                    return Err(ExpressionError::new("Mismatched parentheses"));
                }
            },
        };
    }

    while let Some(operator) = stack.pop() {
        if operator == Token::Left || operator == Token::Right {
            return Err(ExpressionError::new("Mismatched parentheses"));
        }
        output.push(operator);
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::expressions::expression::{Function, Operator};

    use super::*;

    #[test]
    fn simple() {
        let tokens: Vec<Token> = vec![
            Token::Number(10.0),
            Token::Operator(Operator::Addition),
            Token::Number(5.0),
            Token::Operator(Operator::Multiplication),
            Token::Number(2.0),
        ];
        let expected: Vec<Token> = vec![
            Token::Number(10.0),
            Token::Number(5.0),
            Token::Number(2.0),
            Token::Operator(Operator::Multiplication),
            Token::Operator(Operator::Addition),
        ];
        let actual = shunting_yard(tokens);
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn complex() {
        let tokens: Vec<Token> = vec![
            Token::Number(3.0),
            Token::Operator(Operator::Addition),
            Token::Number(4.0),
            Token::Operator(Operator::Multiplication),
            Token::Number(2.0),
            Token::Operator(Operator::Division),
            Token::Left,
            Token::Number(1.0),
            Token::Operator(Operator::Subtraction),
            Token::Number(5.0),
            Token::Right,
            Token::Operator(Operator::Exponentiation),
            Token::Number(2.0),
            Token::Operator(Operator::Exponentiation),
            Token::Number(3.0),
        ];
        let expected: Vec<Token> = vec![
            Token::Number(3.0),
            Token::Number(4.0),
            Token::Number(2.0),
            Token::Operator(Operator::Multiplication),
            Token::Number(1.0),
            Token::Number(5.0),
            Token::Operator(Operator::Subtraction),
            Token::Number(2.0),
            Token::Number(3.0),
            Token::Operator(Operator::Exponentiation),
            Token::Operator(Operator::Exponentiation),
            Token::Operator(Operator::Division),
            Token::Operator(Operator::Addition),
        ];
        let actual = shunting_yard(tokens);
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn functions() {
        let tokens: Vec<Token> = vec![
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
            Token::Number(1.0),
            Token::Right,
        ];
        let expected: Vec<Token> = vec![
            Token::Number(2.0),
            Token::Number(3.0),
            Token::Func(Function::Max),
            Token::Number(3.0),
            Token::Operator(Operator::Division),
            Token::Number(1.0),
            Token::Operator(Operator::Multiplication),
            Token::Func(Function::Sin),
        ];
        let actual = shunting_yard(tokens);
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
