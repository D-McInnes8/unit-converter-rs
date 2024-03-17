use std::ops::Deref;

use log::debug;

use crate::expressions::expression::{Function, Operator};
use crate::expressions::functions::{max, min};

use self::converter::UnitConverter;
use self::expressions::error::ExpressionError;
use self::expressions::expression::AbstractSyntaxTreeNode;

pub mod converter;
pub mod expressions;
mod graph;
mod parser;
pub mod source;

pub struct Expr {
    pub converter: UnitConverter,
}

impl Expr {
    pub fn new(converter: UnitConverter) -> Expr {
        Expr { converter }
    }

    pub fn eval(&mut self, input: &str) -> Result<f64, ExpressionError> {
        let a = self.converter.convert_from_expression("");
        Ok(5.0)
    }
}

pub fn eval_ast(node: &AbstractSyntaxTreeNode, converter: &mut UnitConverter) -> f64 {
    match node {
        AbstractSyntaxTreeNode::Number(num) => num.to_owned(),
        AbstractSyntaxTreeNode::BinaryExpression {
            operator,
            left,
            right,
        } => {
            let left_r = eval_ast(left.as_ref().unwrap().deref(), converter);
            let right_r = eval_ast(right.as_ref().unwrap().deref(), converter);
            let result = match operator {
                Operator::Addition => left_r + right_r,
                Operator::Subtraction => left_r - right_r,
                Operator::Multiplication => left_r * right_r,
                Operator::Division => left_r / right_r,
                Operator::Modulus => left_r % right_r,
                Operator::Exponentiation => left_r.powf(right_r),
                Operator::Conversion => left_r + right_r,
                _ => unreachable!(),
            };
            debug!(
                "Operation: {} {} {} = {}",
                left_r, operator, right_r, result
            );
            result
        }
        AbstractSyntaxTreeNode::UnaryExpression { operator, value } => {
            let val_r = eval_ast(value.as_ref(), converter);
            -val_r
        }
        AbstractSyntaxTreeNode::FunctionExpression { func, expr } => {
            let expr_result = eval_ast(expr, converter);
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
        AbstractSyntaxTreeNode::BinaryConversion { from, to, expr } => unreachable!(),
    }
}
