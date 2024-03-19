use std::collections::HashMap;

use crate::ast::AbstractSyntaxTreeNode;
use crate::error::ExpressionError;
use crate::parser::tokenizer::parse;
use crate::shunting_yard_algorithm::{eval_ast, shunting_yard};

pub struct Expression {
    ast: AbstractSyntaxTreeNode,
    pub expr: String,
    pub ctx: ExpressionContext,
}

impl Expression {
    pub fn new(input: &str) -> Result<Expression, ExpressionError> {
        let expr = input.to_owned();
        let tokens = parse(input)?;
        let ast = shunting_yard(tokens)?;

        Ok(Expression {
            ast,
            expr,
            ctx: ExpressionContext::new(),
        })
    }

    pub fn eval(&self) -> Result<f64, ExpressionError> {
        Ok(eval_ast(&self.ast, &self.ctx))
    }

    pub fn eval_with_ctx(&self, ctx: &ExpressionContext) -> Result<f64, ExpressionError> {
        Ok(eval_ast(&self.ast, ctx))
    }
}

pub struct ExpressionContext {
    pub vars: HashMap<String, f64>,
}

impl ExpressionContext {
    pub fn new() -> ExpressionContext {
        ExpressionContext {
            vars: HashMap::new(),
        }
    }

    pub fn var(&mut self, name: &str, val: f64) -> &mut ExpressionContext {
        self.vars.insert(name.to_owned(), val);
        self
    }
}
