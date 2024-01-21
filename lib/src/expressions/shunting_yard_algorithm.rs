use std::collections::VecDeque;
use std::rc::Rc;

use log::debug;

use super::expression::{OperationType, Operator};
use super::tokenizer::Token;

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

fn shunting_yard(tokens: Vec<Token>) {
    let mut output: VecDeque<f64> = VecDeque::new();
    let mut operator: Vec<Token> = vec![];

    for token in tokens {
        match token {
            Token::Left => {}
            Token::Right => {}
            Token::Operator { value } => {}
            Token::Number { value } => {
                output.push_back(value);
            }
            Token::Func { value } => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
