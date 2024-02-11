use std::fmt::Display;

use log::{info, trace};

use crate::parser::tokenizer::parse;

use super::error::ExpressionError;
use super::shunting_yard_algorithm::{eval_ast, shunting_yard};

#[derive(Debug, PartialEq)]
pub enum AbstractSyntaxTreeNode {
    Number(f64),
    BinaryExpression {
        operator: Operator,
        left: Option<Box<AbstractSyntaxTreeNode>>,
        right: Option<Box<AbstractSyntaxTreeNode>>,
    },
    UnaryExpression {
        operator: Operator,
        value: Box<AbstractSyntaxTreeNode>,
    },
    FunctionExpression {
        func: Function,
        expr: Box<AbstractSyntaxTreeNode>,
    },
    FunctionParams {
        func: Function,
        params: Vec<AbstractSyntaxTreeNode>,
    },
}

#[derive(Debug, PartialEq)]
pub enum FunctionValue {
    Expression(Box<AbstractSyntaxTreeNode>),
    List(Vec<Box<AbstractSyntaxTreeNode>>),
}

impl Display for AbstractSyntaxTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_ast_node(self, f, String::new(), String::new())
    }
}

fn fmt_ast_node(
    node: &AbstractSyntaxTreeNode,
    f: &mut std::fmt::Formatter<'_>,
    prefix: String,
    children_prefix: String,
) -> std::fmt::Result {
    write!(f, "{}", prefix)?;
    match node {
        AbstractSyntaxTreeNode::Number(num) => writeln!(f, "{}", num),
        AbstractSyntaxTreeNode::BinaryExpression {
            operator,
            left,
            right,
        } => {
            writeln!(f, "{:?}", operator)?;
            fmt_ast_node(
                left.as_ref().unwrap(),
                f,
                children_prefix.clone() + "├── ",
                children_prefix.clone() + "│   ",
            )?;
            fmt_ast_node(
                right.as_ref().unwrap(),
                f,
                children_prefix.clone() + "└── ",
                children_prefix.clone() + "    ",
            )
        }
        AbstractSyntaxTreeNode::UnaryExpression { operator, value } => {
            writeln!(f, "{:?}", operator)?;
            fmt_ast_node(
                value,
                f,
                children_prefix.clone() + "└── ",
                children_prefix.clone() + "    ",
            )
        }
        AbstractSyntaxTreeNode::FunctionExpression { func, expr } => {
            writeln!(f, "{:?}", func)?;
            fmt_ast_node(
                expr,
                f,
                children_prefix.clone() + "└── ",
                children_prefix.clone() + "    ",
            )
        }
        AbstractSyntaxTreeNode::FunctionParams { func, params } => {
            writeln!(f, "{:?}", func)?;
            for (i, param) in params.iter().enumerate() {
                if i >= params.len() - 1 {
                    fmt_ast_node(
                        param,
                        f,
                        children_prefix.clone() + "└── ",
                        children_prefix.clone() + "    ",
                    )?;
                } else {
                    fmt_ast_node(
                        param,
                        f,
                        children_prefix.clone() + "├── ",
                        children_prefix.clone() + "│   ",
                    )?;
                }
            }
            write!(f, "")
        }
    }
}

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
    Conversion,
    Negative,
    Assignment,
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
            Operator::Conversion => write!(f, "->"),
            Operator::Negative => write!(f, "-"),
            Operator::Assignment => write!(f, "="),
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
            | Operator::Modulus
            | Operator::Conversion => Associativity::Left,
            Operator::Exponentiation | Operator::Negative | Operator::Assignment => {
                Associativity::Right
            }
        }
    }

    pub const fn prec(self) -> u32 {
        match self {
            Operator::Assignment => 0,
            Operator::Conversion => 1,
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

    Ok(eval_ast(&ast))
}
