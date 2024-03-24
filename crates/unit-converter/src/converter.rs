use crate::graph::Graph;
use crate::parser::{parse_conversion, UnitAbbreviation};
use expr::expression::Expression;
use log::{error, info, warn};

use self::builder::UnitConverterBuilder;
use self::error::ConversionError;

pub mod builder;
pub mod error;

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
            let n0 = self.graph[graph_index]
                .get_node_index(from.to_string())
                .ok_or(ConversionError::new(
                    format!("Unable to find conversion for unit {}", from).as_str(),
                ))?;

            let n1 = self.graph[graph_index]
                .get_node_index(to.to_string())
                .ok_or(ConversionError::new(
                    format!("Unable to find conversion for unit {}", to).as_str(),
                ))?;

            let shortest_path = self.graph[graph_index].shortest_path(n0, n1);
            info!(
                "Converting from {} to {} will require {} operation(s)",
                from,
                to,
                shortest_path.len()
            );

            let mut multiplier = 1.0;
            let mut any_expr: bool = false;
            for (_, conversion) in &shortest_path {
                match conversion {
                    Conversion::Multiplier(val) => {
                        multiplier *= val;
                    }
                    Conversion::Expression(_) => {
                        any_expr = true;
                    }
                }
            }

            //let multiplier = calculate_conversion_multiplier(&shortest_path);
            let return_value = value * multiplier;

            if self.cache && !any_expr && shortest_path.len() > 1 {
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

            return Ok(return_value);
        }

        error!("Unable to get internal graph for unit type {}", unit_type);
        Err(ConversionError::new(
            "Unable to get internal graph for unit type",
        ))
    }

    pub fn units(&self) -> &Vec<UnitAbbreviation> {
        &self.abbreviations
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
