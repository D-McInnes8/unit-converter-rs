//use std::fmt::write;
use std::fmt::Display;
use std::ops::Deref;

use super::error::ExpressionError;
use super::shunting_yard_algorithm::{eval_ast, eval_rpn, shunting_yard};
use super::tokenizer::get_tokens;

#[derive(Debug, PartialEq)]
pub enum AbstractSyntaxTreeNode {
    Number(f64),
    BinaryExpression {
        operator: Operator,
        left: Option<Box<AbstractSyntaxTreeNode>>,
        right: Option<Box<AbstractSyntaxTreeNode>>,
    },
    Function {
        func: Function,
        value: FunctionValue,
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

/*impl AbstractSyntaxTreeNode {
    pub fn print(&self, buffer: &mut String) {
        match self {
            Self::Number(num) => buffer.push_str(format!("{}", num)),
            Self::BinaryExpression {
                operator,
                left,
                right,
            } => write!(buffer, ""),
            Self::Function { func, value } => write!(buffer, ""),
        }
    }
}*/

impl Display for AbstractSyntaxTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /*match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::BinaryExpression {
                operator,
                left,
                right,
            } => {
                let l = left.as_ref().unwrap().deref();
                let r = right.as_ref().unwrap().deref();
                write!(f, "{:?}\n ├── {}\n └── {}", operator, l, r)
            }
            Self::Function { func, value } => write!(f, "{:?}: {}", func, ""),
        }*/
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
        AbstractSyntaxTreeNode::Function { func, value } => {
            writeln!(f, "{:?}", func)?;
            if let FunctionValue::Expression(exp) = value {
                fmt_ast_node(
                    exp,
                    f,
                    children_prefix.clone() + "└── ",
                    children_prefix.clone() + "    ",
                )?;
            } else if let FunctionValue::List(params) = value {
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
            }
            write!(f, "")
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
    eprintln!();
    eprintln!("Expression: {}", input);
    eprintln!();
    eprintln!();

    let tokens = get_tokens(input)?;
    let ast = shunting_yard(tokens)?;

    eprintln!();
    eprintln!();
    eprintln!("AST:\n {}", ast);
    eprintln!();
    eprintln!("{:?}", ast);
    eprintln!();

    Ok(eval_ast(&ast))
}
