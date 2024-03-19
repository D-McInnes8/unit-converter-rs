use std::collections::HashMap;

use crate::ast::AbstractSyntaxTreeNode;
use crate::error::ExpressionError;
use crate::parser::tokenizer::parse;
use crate::shunting_yard_algorithm::{eval_ast, shunting_yard};

pub struct Expression {
    ast: AbstractSyntaxTreeNode,
    pub expr: String,
    pub ctx: InMemoryExpressionContext,
}

impl Expression {
    pub fn new(input: &str) -> Result<Expression, ExpressionError> {
        let expr = input.to_owned();
        let tokens = parse(input)?;
        let ast = shunting_yard(tokens)?;

        Ok(Expression {
            ast,
            expr,
            ctx: InMemoryExpressionContext::new(),
        })
    }

    pub fn eval(&self) -> Result<f64, ExpressionError> {
        Ok(eval_ast(&self.ast, &self.ctx))
    }

    pub fn eval_with_ctx(&self, ctx: &impl ExpressionContext) -> Result<f64, ExpressionError> {
        Ok(eval_ast(&self.ast, ctx))
    }
}

pub trait ExpressionContext {
    fn get(&self, name: &str) -> Option<f64>;
    fn var(&mut self, name: &str, val: f64);
}

pub struct InMemoryExpressionContext {
    pub vars: HashMap<String, f64>,
}

impl InMemoryExpressionContext {
    pub fn new() -> InMemoryExpressionContext {
        InMemoryExpressionContext {
            vars: HashMap::new(),
        }
    }
}

impl ExpressionContext for InMemoryExpressionContext {
    fn get(&self, name: &str) -> Option<f64> {
        self.vars.get(name).map(|m| m.to_owned())
    }

    fn var(&mut self, name: &str, val: f64) {
        self.vars.insert(name.to_owned(), val);
    }
}
