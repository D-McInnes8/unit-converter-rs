use crate::graph::Graph;
use crate::parser::{parse_conversion, UnitAbbreviation};
use expr::expression::ExpressionContext;
use expr::expression::{Expression, InMemoryExpressionContext};
use log::{debug, error, info, warn};

use self::builder::UnitConverterBuilder;
use self::error::ConversionError;

pub mod builder;
pub mod error;

// TODO: Give these structs more unique names, rather than them all being some variation of
// Converter/Conversion.
pub enum Conversion {
    Multiplier(f64),
    Expression(Expression),
}

pub struct UnitConverter {
    graph: Vec<Graph<String, Conversion>>,
    abbreviations: Vec<UnitAbbreviation>,
    cache: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnitConversion {
    pub value: f64,
    pub from: String,
    pub to: String,
    pub unit_type: String,
}

impl UnitConverter {
    pub fn builder() -> UnitConverterBuilder {
        UnitConverterBuilder::new()
    }

    pub fn new(
        graph: Vec<Graph<String, Conversion>>,
        abbreviations: Vec<UnitAbbreviation>,
        cache: bool,
    ) -> UnitConverter {
        UnitConverter {
            graph,
            abbreviations,
            cache,
        }
    }

    pub fn unit_info(&self, input: &str) -> Result<UnitAbbreviation, ConversionError> {
        let lc_input = input.to_lowercase();
        for abbrev in &self.abbreviations {
            if abbrev.unit.to_lowercase() == lc_input || abbrev.abbrev == lc_input {
                return Ok(abbrev.to_owned());
            }
        }

        // TODO: Return an actual error here rather than just default().
        Err(ConversionError::default())
    }

    pub fn convert_from_expression(
        &mut self,
        input: &str,
    ) -> Result<UnitConversion, ConversionError> {
        match parse_conversion(&self.abbreviations, input) {
            Ok(conversion) => {
                info!("Parsed {:?}", conversion);
                let result = self.convert_from_definition(
                    &conversion.unit_type,
                    &conversion.from,
                    &conversion.to,
                    conversion.value,
                )?;
                Ok(UnitConversion {
                    value: result,
                    from: conversion.from,
                    to: conversion.to,
                    unit_type: conversion.unit_type,
                })
            }
            Err(err) => Err(err),
        }
    }

    pub fn convert_from_definition(
        &mut self,
        unit_type: &str,
        from: &str,
        to: &str,
        value: f64,
    ) -> Result<f64, ConversionError> {
        if let Some(graph_index) = self.get_graph_index(unit_type) {
            let n0 = self.get_graph_node_index(graph_index, from)?;
            let n1 = self.get_graph_node_index(graph_index, to)?;

            let shortest_path = self.graph[graph_index].shortest_path(n0, n1);
            if shortest_path.is_empty() {
                return Err(ConversionError::new("Unable to find conversion"));
            }

            debug!(
                "Converting from {} to {} will require {} operation(s)",
                from,
                to,
                shortest_path.len()
            );

            // TODO: Refactor this whole section of code.
            let mut multiplier = 1.0;
            let mut result_val = value;
            let mut should_cache_multiplier: bool = true;

            for edge in &shortest_path {
                match edge.weight {
                    Conversion::Multiplier(val) => {
                        multiplier *= val;
                    }
                    Conversion::Expression(expr) => {
                        should_cache_multiplier = false;
                        result_val *= multiplier;
                        multiplier = 1.0;

                        let mut ctx = InMemoryExpressionContext::default();
                        let params = self
                            .get_unit_abbrev(edge.source, unit_type)
                            .ok_or(ConversionError::new("Unable to find unit."))?;
                        ctx.var(params, result_val);

                        result_val = expr.eval_with_ctx(&ctx)?;
                    }
                }
            }

            result_val *= multiplier;

            // Should cache the multiplier only if all conversions were multiplier conversions and
            // if there length of the path is greater than 1.
            if self.cache && should_cache_multiplier && shortest_path.len() > 1 {
                info!(
                    "Caching conversion between {} and {} using multiplier {}",
                    from, to, multiplier
                );
                let cache_result =
                    self.graph[graph_index].add_edge(n0, n1, Conversion::Multiplier(multiplier));
                if cache_result.is_err() {
                    warn!(
                        "Unable to add edge to graph between nodes {} and {}",
                        from, to
                    );
                }
            }

            return Ok(result_val);
        }

        error!("Unable to get internal graph for unit type {}", unit_type);
        Err(ConversionError::new(
            "Unable to get internal graph for unit type",
        ))
    }

    pub fn units(&self) -> &Vec<UnitAbbreviation> {
        &self.abbreviations
    }

    fn get_unit_abbrev(&self, unit: &str, unit_type: &str) -> Option<&str> {
        for def in &self.abbreviations {
            if def.unit == unit && def.unit_type == unit_type {
                return Some(&def.abbrev);
            }
        }
        None
    }

    fn get_graph_node_index(
        &self,
        graph_index: usize,
        node: &str,
    ) -> Result<usize, ConversionError> {
        self.graph[graph_index]
            .get_node_index(node.to_string())
            .ok_or(ConversionError::new(
                format!("Unable to find definition for unit {}", node).as_str(),
            ))
    }

    fn get_graph_index(&self, category: &str) -> Option<usize> {
        for (i, graph) in self.graph.iter().enumerate() {
            if graph.id == category {
                return Some(i);
            }
        }
        None
    }
}
