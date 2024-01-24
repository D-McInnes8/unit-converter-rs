use std::rc::Rc;

use super::error::ParseError;
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

pub fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, ParseError> {
    let mut output = Vec::with_capacity(tokens.len());
    let mut stack: Vec<Token> = Vec::with_capacity(tokens.len());

    for token in tokens {
        println!("Checking token {:?}", token);
        match token {
            Token::Number(_) => {
                println!("Pushing number to output {:?}", token);
                output.push(token);
            }
            Token::Operator(o1) => loop {
                if let Some(o2) = stack.last() {
                    println!("o1: {:?}, o2: {:?}", o1, o2);
                    match *o2 {
                        Token::Left => {
                            println!("Pushing left token to stack");
                            stack.push(token);
                            break;
                        }
                        Token::Operator(o2)
                            if (o2.prec() > o1.prec())
                                || (o2.prec() == o1.prec()
                                    && o1.assoc() == Associativity::Left) =>
                        {
                            println!(
                                "Popping {:?} of the stack and pushing onto output queue",
                                o2
                            );
                            output.push(stack.pop().unwrap());
                        }
                        _ => {
                            println!("Pushing {:?} token to stack B", token);
                            stack.push(token);
                            break;
                        }
                    }
                } else {
                    println!("Pushing {:?} token to stack A", token);
                    stack.push(token);
                    break;
                }
            },
            Token::Func(_) => {}
            Token::Left => {
                println!("Pushing left token onto the stack (match)");
                stack.push(token);
            }
            Token::Right => loop {
                println!("Stack: {:?}", stack);
                if let Some(top) = stack.last() {
                    if *top == Token::Left {
                        println!("Popping top of stack and discarding left paren");
                        _ = stack.pop();
                        break;
                    }

                    println!("Popping {:?} from stack and pushing to output queue", top);
                    output.push(stack.pop().unwrap());
                } else {
                    let err: Option<ParseError> = None;
                    return Err(ParseError::new("Mismatched parentheses", "", err));
                }
            },
        };
    }

    println!("{:?}", stack);

    while let Some(operator) = stack.pop() {
        if operator == Token::Left || operator == Token::Right {
            let err: Option<ParseError> = None;
            return Err(ParseError::new("", "", err));
        }
        println!("Pushing operator {:?} onto output queue", operator);
        output.push(operator);
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::expressions::expression::Operator;

    use super::*;

    #[test]
    fn shunting_yard_algorithm() {
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
    fn shunting2() {
        let tokens: Vec<Token> = vec![
            Token::Number(10.0),
            Token::Operator(Operator::Addition),
            Token::Number(5.0),
            Token::Operator(Operator::Multiplication),
            Token::Number(2.0),
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
