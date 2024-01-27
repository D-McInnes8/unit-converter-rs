use super::error::ExpressionError;
use super::shunting_yard_algorithm::{eval_rpn, shunting_yard};
use super::tokenizer::get_tokens;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperationType {
    Number(f64),
    BinaryExpression { operator: Operator },
    Function { name: Function, value: f64 },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
    Modulus,
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
            Operator::Exponentiation => Associativity::Right,
        }
    }

    pub const fn prec(self) -> u32 {
        match self {
            Operator::Addition | Operator::Subtraction => 2,
            Operator::Multiplication | Operator::Division | Operator::Modulus => 3,
            Operator::Exponentiation => 4,
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
    let tokens = get_tokens(input)?;
    let rpn = shunting_yard(tokens)?;
    eval_rpn(rpn)
}
