use super::shunting_yard_algorithm::{shunting_yard, Associativity};
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
}

impl Operator {
    pub const fn assoc(self) -> Associativity {
        match self {
            Operator::Addition
            | Operator::Subtraction
            | Operator::Multiplication
            | Operator::Division => Associativity::Left,
            Operator::Exponentiation => Associativity::Right,
        }
    }

    pub const fn prec(self) -> u32 {
        match self {
            Operator::Addition => 2,
            Operator::Subtraction => 2,
            Operator::Multiplication => 3,
            Operator::Division => 3,
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

pub fn evaluate_expression(input: &str, value: f64) -> Option<f64> {
    let tokens = get_tokens(input).unwrap();
    let ast = shunting_yard(tokens);

    None
}
