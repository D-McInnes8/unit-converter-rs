use std::fmt::Display;

use crate::{Function, Operator};

#[derive(Debug, PartialEq)]
pub enum AbstractSyntaxTreeNode {
    Number(f64),
    Variable(String),
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
        AbstractSyntaxTreeNode::Variable(var) => writeln!(f, "{}", var),
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
