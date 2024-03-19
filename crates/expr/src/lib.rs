use std::fmt::Display;

use log::{info, trace};

use crate::expression::{ExpressionContext, InMemoryExpressionContext};
use crate::parser::tokenizer::parse;
use crate::shunting_yard_algorithm::{eval_ast, shunting_yard};

use self::error::ExpressionError;

mod ast;
pub mod error;
pub mod expression;
mod functions;
pub mod parser;
mod shunting_yard_algorithm;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
    Modulus,
    Negative,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Addition => write!(f, "+"),
            Operator::Subtraction => write!(f, "-"),
            Operator::Multiplication => write!(f, "*"),
            Operator::Division => write!(f, "/"),
            Operator::Exponentiation => write!(f, "^"),
            Operator::Modulus => write!(f, "%"),
            Operator::Negative => write!(f, "-"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
}

impl Operator {
    pub const fn assoc(self) -> Associativity {
        match self {
            Operator::Addition
            | Operator::Subtraction
            | Operator::Multiplication
            | Operator::Division
            | Operator::Modulus => Associativity::Left,
            Operator::Exponentiation | Operator::Negative => Associativity::Right,
        }
    }

    pub const fn prec(self) -> u32 {
        match self {
            Operator::Addition | Operator::Subtraction => 2,
            Operator::Multiplication | Operator::Division | Operator::Modulus => 3,
            Operator::Exponentiation | Operator::Negative => 4,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Function {
    Max,
    Min,
    Sin,
    Cos,
    Tan,
}

pub fn eval(input: &str) -> Result<f64, ExpressionError> {
    info!("Parsing expression {}", input);

    //let tokens = get_tokens(input)?;
    let tokens = parse(input)?;
    info!("Parsed {} tokens from expression", tokens.len());
    trace!("{:?}", tokens);

    let ast = shunting_yard(tokens)?;
    info!("Generating abstract syntax tree");
    trace!("\n{}", ast);

    let ctx = InMemoryExpressionContext::new();
    Ok(eval_ast(&ast, &ctx))
}
