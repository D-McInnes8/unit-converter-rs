use crate::graph::Graph;
use crate::parser::{parse_conversion, UnitAbbreviation};
use log::{debug, error, info};

use self::builder::UnitConverterBuilder;
use self::error::ConversionError;

pub mod builder;
pub mod error;

pub struct UnitConverter {
    graph: Vec<Graph<String, f64>>,
    abbreviations: Vec<UnitAbbreviation>,
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
        graph: Vec<Graph<String, f64>>,
        abbreviations: Vec<UnitAbbreviation>,
    ) -> UnitConverter {
        UnitConverter {
            graph: graph,
            abbreviations: abbreviations,
        }
    }

    pub fn convert_from_expression(&mut self, input: &str) -> Result<f64, ConversionError> {
        match parse_conversion(&self.abbreviations, &input) {
            Ok(conversion) => {
                info!("Parsed {:?}", conversion);
                return self.convert_from_definition(
                    &conversion.unit_type,
                    &conversion.from,
                    &conversion.to,
                    conversion.value,
                );
            }
            Err(err) => {
                return Err(err);
            }
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
                "Converting from {} to {} will require {} operations",
                from,
                to,
                shortest_path.len()
            );

            let mut return_value = value;
            for (unit, conversion) in &shortest_path {
                debug!(
                    "Converting value to {} ({} *= {})",
                    unit, return_value, conversion
                );
                return_value *= *conversion;
            }

            if shortest_path.len() > 1 {
                let mut conversion_value = 1.0;
                for (_, conversion) in &shortest_path {
                    conversion_value *= *conversion;
                }

                info!(
                    "Caching conversion between {} and {} with {}",
                    from, to, conversion_value
                );
                _ = self.graph[graph_index].add_edge(n0, n1, conversion_value);
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
        let mut i = 0;
        for graph in &self.graph {
            if graph.id == category {
                return Some(i);
            }
            i += 1;
        }
        None
    }
}
