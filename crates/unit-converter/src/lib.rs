use std::fmt::Display;

pub mod converter;
mod graph;
mod parser;
pub mod source;

pub struct ConversionDefinition {
    category: String,
    from: String,
    to: String,
    val: ConversionValueDefinition,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConversionValueDefinition {
    Multiplier(f64),
    Expression(String),
}

impl Display for ConversionValueDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Multiplier(val) => f.write_fmt(format_args!("{}", val)),
            Self::Expression(expr) => f.write_str(expr),
        }
    }
}
