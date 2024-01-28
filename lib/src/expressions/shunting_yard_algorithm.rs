use std::ops::Deref;

use log::{debug, error, trace};

use super::error::ExpressionError;
use super::expression::{AbstractSyntaxTreeNode, Associativity, Function, Operator};
use super::tokenizer::Token;

pub fn eval_ast(node: &AbstractSyntaxTreeNode) -> f64 {
    match node {
        AbstractSyntaxTreeNode::Number(num) => num.to_owned(),
        AbstractSyntaxTreeNode::BinaryExpression {
            operator,
            left,
            right,
        } => {
            let left_r = eval_ast(left.as_ref().unwrap().deref());
            let right_r = eval_ast(right.as_ref().unwrap().deref());
            let result = match operator {
                Operator::Addition => left_r + right_r,
                Operator::Subtraction => left_r - right_r,
                Operator::Multiplication => left_r * right_r,
                Operator::Division => left_r / right_r,
                Operator::Modulus => left_r % right_r,
                Operator::Exponentiation => left_r.powf(right_r),
            };
            debug!(
                "Operation: {} {} {} = {}",
                left_r, operator, right_r, result
            );
            result
        }
        AbstractSyntaxTreeNode::FunctionExpression { func, expr } => {
            let expr_result = eval_ast(expr);
            let result = match func {
                Function::Sin => expr_result.sin(),
                Function::Cos => expr_result.cos(),
                Function::Tan => expr_result.tan(),
                _ => unreachable!(),
            };
            debug!(
                "Applying Function {:?} ({:?}) = {}",
                func, expr_result, result
            );
            result
        }
        AbstractSyntaxTreeNode::FunctionParams { func, params } => {
            let result = match func {
                Function::Max => max(params).unwrap(),
                Function::Min => min(params).unwrap(),
                _ => unreachable!(),
            };
            debug!("Applying Function {:?} ({:?}) = {}", func, params, result);
            result
        }
    }
}

fn max(params: &Vec<AbstractSyntaxTreeNode>) -> Option<f64> {
    let mut result = None;
    for param in params {
        let ev = eval_ast(param);
        if let Some(n) = result {
            if ev > n {
                result = Some(ev);
            }
        } else {
            result = Some(ev);
        }
    }
    result
}

fn min(params: &Vec<AbstractSyntaxTreeNode>) -> Option<f64> {
    let mut result = None;
    for param in params {
        let ev = eval_ast(param);
        if let Some(n) = result {
            if ev < n {
                result = Some(ev);
            }
        } else {
            result = Some(ev);
        }
    }
    result
}

pub fn eval_rpn(tokens: Vec<Token>) -> Result<f64, ExpressionError> {
    let mut stack = Vec::with_capacity(tokens.len());

    eprintln!("Tokens: {:?}", &tokens);

    for token in &tokens {
        if let Token::Number(n) = token {
            stack.push(*n);
            continue;
        }

        let right = stack.pop();
        let left = stack.pop();

        eprintln!("Right: {:?}, Left: {:?}", right, left);

        match (left, right) {
            (Some(a), Some(b)) => {
                let result = match token {
                    Token::Operator(Operator::Addition) => a + b,
                    Token::Operator(Operator::Subtraction) => a - b,
                    Token::Operator(Operator::Multiplication) => a * b,
                    Token::Operator(Operator::Division) => a / b,
                    Token::Operator(Operator::Exponentiation) => a.powf(b),
                    Token::Operator(Operator::Modulus) => a % b,
                    Token::Func(func) => {
                        eprintln!("Func: {:?}", func);
                        eprintln!("Right: {:?}", right);
                        eprintln!("Left: {:?}", left);
                        eprintln!("Output: {:?}", &tokens);
                        unreachable!();
                    }
                    _ => unreachable!(),
                };
                stack.push(result.to_owned());
            }
            (None, Some(b)) => {
                if let Token::Func(func) = token {
                    let result = match func {
                        Function::Sin => b.sin(),
                        Function::Cos => b.cos(),
                        Function::Tan => b.tan(),
                        _ => return Ok(b),
                    };
                    stack.push(result.to_owned());
                } else {
                    return Ok(b);
                }
            }
            (None, None) | (Some(_), None) => unreachable!(),
        }
    }

    stack.pop().map_or_else(
        || Err(ExpressionError::new("An unknown error has occured")),
        Ok,
    )
}

fn pop_to_output_queue(token: Token, output: &mut Vec<AbstractSyntaxTreeNode>) {
    debug!("Popping operator {:?} from stack to output queue", token);
    trace!("{} items in output queue {:?}", output.len(), output);
    if let Token::Number(num) = token {
        output.push(AbstractSyntaxTreeNode::Number(num));
    } else if let Token::Func(func) = token {
        let mut params = vec![];
        while let Some(param) = output.last() {
            match param {
                AbstractSyntaxTreeNode::Number(_) => params.push(output.pop().unwrap()),
                _ => break,
            };
        }

        match params.len() {
            2.. => output.push(AbstractSyntaxTreeNode::FunctionParams { func, params }),
            1 => output.push(AbstractSyntaxTreeNode::FunctionExpression {
                func,
                expr: Box::new(params.remove(0)),
            }),
            _ => {
                if let Some(expr) = output.pop() {
                    output.push(AbstractSyntaxTreeNode::FunctionExpression {
                        func,
                        expr: Box::new(expr),
                    })
                } else {
                    unreachable!();
                }
            }
        }
    } else if let Token::Operator(operator) = token {
        let right = output.pop().map_or_else(|| None, |a| Some(Box::new(a)));
        let left = output.pop().map_or_else(|| None, |a| Some(Box::new(a)));

        output.push(AbstractSyntaxTreeNode::BinaryExpression {
            operator,
            left,
            right,
        })
    }
}

pub fn shunting_yard(tokens: Vec<Token>) -> Result<AbstractSyntaxTreeNode, ExpressionError> {
    let mut output = Vec::with_capacity(tokens.len());
    let mut stack: Vec<Token> = Vec::with_capacity(tokens.len());

    for token in tokens {
        debug!("Analyising token {:?}", token);
        match token {
            Token::Number(num) => {
                output.push(AbstractSyntaxTreeNode::Number(num));
            }
            Token::Func(_) => {
                stack.push(token);
            }
            Token::Operator(o1) => {
                while let Some(o2) = stack.last() {
                    match *o2 {
                        Token::Operator(o2)
                            if (o2.prec() > o1.prec())
                                || (o2.prec() == o1.prec()
                                    && o1.assoc() == Associativity::Left) =>
                        {
                            pop_to_output_queue(stack.pop().unwrap(), &mut output);
                        }
                        _ => {
                            break;
                        }
                    }
                }
                stack.push(token);
            }
            Token::Comma => {
                while let Some(top) = stack.last() {
                    if *top != Token::Left {
                        pop_to_output_queue(stack.pop().unwrap(), &mut output);
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
                            pop_to_output_queue(stack.pop().unwrap(), &mut output);
                        }

                        break;
                    }

                    pop_to_output_queue(stack.pop().unwrap(), &mut output);
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
        pop_to_output_queue(operator, &mut output);
    }

    if output.len() > 1 {
        error!(
            "Output queue contains {} items. Should only contain 1",
            output.len()
        );
        debug!("{:?}", output);
        debug!("\n{}", output[1]);
        return Err(ExpressionError::new(
            "Output queue contains more than 1 item",
        ));
    }
    Ok(output.remove(0))
}

#[cfg(test)]
mod tests {
    use crate::expressions::expression::{Function, Operator};
    use crate::expressions::tokenizer::get_tokens;

    use super::*;

    #[test]
    fn single_binary_expression() {
        let expected = AbstractSyntaxTreeNode::BinaryExpression {
            operator: Operator::Addition,
            left: Some(Box::new(AbstractSyntaxTreeNode::Number(5.0))),
            right: Some(Box::new(AbstractSyntaxTreeNode::Number(10.0))),
        };
        let tokens = get_tokens("5 + 10").unwrap();
        let actual = shunting_yard(tokens).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplication_precedence() {
        let tokens: Vec<Token> = vec![
            Token::Number(10.0),
            Token::Operator(Operator::Addition),
            Token::Number(5.0),
            Token::Operator(Operator::Multiplication),
            Token::Number(2.0),
        ];
        let expected = AbstractSyntaxTreeNode::BinaryExpression {
            operator: Operator::Addition,
            left: Some(Box::new(AbstractSyntaxTreeNode::Number(10.0))),
            right: Some(Box::new(AbstractSyntaxTreeNode::BinaryExpression {
                operator: Operator::Multiplication,
                left: Some(Box::new(AbstractSyntaxTreeNode::Number(5.0))),
                right: Some(Box::new(AbstractSyntaxTreeNode::Number(2.0))),
            })),
        };

        let actual = shunting_yard(tokens).expect("");
        assert_eq!(expected, actual);
    }

    #[test]
    fn complex() {
        let tokens: Vec<Token> = vec![
            Token::Number(3.0),
            Token::Operator(Operator::Addition),
            Token::Number(4.0),
            Token::Operator(Operator::Multiplication),
            Token::Left,
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
            Token::Right,
        ];

        let expected = AbstractSyntaxTreeNode::BinaryExpression {
            operator: Operator::Addition,
            left: Some(Box::new(AbstractSyntaxTreeNode::Number(3.0))),
            right: Some(Box::new(AbstractSyntaxTreeNode::BinaryExpression {
                operator: Operator::Multiplication,
                left: Some(Box::new(AbstractSyntaxTreeNode::Number(4.0))),
                right: Some(Box::new(AbstractSyntaxTreeNode::BinaryExpression {
                    operator: Operator::Division,
                    left: Some(Box::new(AbstractSyntaxTreeNode::Number(2.0))),
                    right: Some(Box::new(AbstractSyntaxTreeNode::BinaryExpression {
                        operator: Operator::Exponentiation,
                        left: Some(Box::new(AbstractSyntaxTreeNode::BinaryExpression {
                            operator: Operator::Subtraction,
                            left: Some(Box::new(AbstractSyntaxTreeNode::Number(1.0))),
                            right: Some(Box::new(AbstractSyntaxTreeNode::Number(5.0))),
                        })),
                        right: Some(Box::new(AbstractSyntaxTreeNode::BinaryExpression {
                            operator: Operator::Exponentiation,
                            left: Some(Box::new(AbstractSyntaxTreeNode::Number(2.0))),
                            right: Some(Box::new(AbstractSyntaxTreeNode::Number(3.0))),
                        })),
                    })),
                })),
            })),
        };

        let actual = shunting_yard(tokens).expect("");
        assert_eq!(expected, actual);
    }

    #[test]
    fn function_simple() {
        let tokens: Vec<Token> = vec![
            Token::Func(Function::Sin),
            Token::Left,
            Token::Number(2.0),
            Token::Operator(Operator::Addition),
            Token::Number(1.0),
            Token::Right,
            Token::Operator(Operator::Exponentiation),
            Token::Number(10.0),
        ];

        let expected = AbstractSyntaxTreeNode::BinaryExpression {
            operator: Operator::Exponentiation,
            left: Some(Box::new(AbstractSyntaxTreeNode::FunctionExpression {
                func: Function::Sin,
                expr: Box::new(AbstractSyntaxTreeNode::BinaryExpression {
                    operator: Operator::Addition,
                    left: Some(Box::new(AbstractSyntaxTreeNode::Number(2.0))),
                    right: Some(Box::new(AbstractSyntaxTreeNode::Number(1.0))),
                }),
            })),
            right: Some(Box::new(AbstractSyntaxTreeNode::Number(10.0))),
        };
        let actual = shunting_yard(tokens).expect("");

        assert_eq!(expected, actual);
    }

    #[test]
    fn function_with_parameter_list() {
        let tokens: Vec<Token> = vec![
            Token::Func(Function::Max),
            Token::Left,
            Token::Number(2.0),
            Token::Comma,
            Token::Number(3.0),
            Token::Comma,
            Token::Number(1.0),
            Token::Right,
            Token::Operator(Operator::Exponentiation),
            Token::Number(10.0),
        ];

        let expected = AbstractSyntaxTreeNode::BinaryExpression {
            operator: Operator::Exponentiation,
            left: Some(Box::new(AbstractSyntaxTreeNode::FunctionParams {
                func: Function::Max,
                params: vec![
                    AbstractSyntaxTreeNode::Number(1.0),
                    AbstractSyntaxTreeNode::Number(3.0),
                    AbstractSyntaxTreeNode::Number(2.0),
                ],
            })),
            right: Some(Box::new(AbstractSyntaxTreeNode::Number(10.0))),
        };
        let actual = shunting_yard(tokens).expect("");
        eprintln!("Expected: {}", expected);
        eprintln!("Actual: {}", actual);
        assert_eq!(expected, actual);
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

        let expected = AbstractSyntaxTreeNode::FunctionExpression {
            func: Function::Sin,
            expr: Box::new(AbstractSyntaxTreeNode::BinaryExpression {
                operator: Operator::Multiplication,
                left: Some(Box::new(AbstractSyntaxTreeNode::BinaryExpression {
                    operator: Operator::Division,
                    left: Some(Box::new(AbstractSyntaxTreeNode::FunctionParams {
                        func: Function::Max,
                        params: vec![
                            AbstractSyntaxTreeNode::Number(3.0),
                            AbstractSyntaxTreeNode::Number(2.0),
                        ],
                    })),
                    right: Some(Box::new(AbstractSyntaxTreeNode::Number(3.0))),
                })),
                right: Some(Box::new(AbstractSyntaxTreeNode::Number(1.0))),
            }),
        };

        let actual = shunting_yard(tokens).expect("");
        assert_eq!(expected, actual);
    }
}
