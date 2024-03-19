use crate::ast::AbstractSyntaxTreeNode;
use crate::expression::ExpressionContext;

use super::shunting_yard_algorithm::eval_ast;

pub fn max(params: &Vec<AbstractSyntaxTreeNode>, ctx: &impl ExpressionContext) -> Option<f64> {
    let mut result = None;
    for param in params {
        let ev = eval_ast(param, ctx);
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

pub fn min(params: &Vec<AbstractSyntaxTreeNode>, ctx: &impl ExpressionContext) -> Option<f64> {
    let mut result = None;
    for param in params {
        let ev = eval_ast(param, ctx);
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
