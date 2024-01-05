use log::{error, info};

use crate::graph::Graph;
use crate::parser::{parse_conversion, UnitAbbreviation};

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
        if let Some(graph) = self.get_graph_internal(unit_type) {
            let n0 = graph.get_node_index(from.to_string()).unwrap();
            let n1 = graph.get_node_index(to.to_string()).unwrap();
            let shortest_path = graph.shortest_path(n0, n1);

            let mut return_value = value;
            for (unit, conversion) in shortest_path {
                info!(
                    "Converting value to {}. Expresion is {} *= {}",
                    unit, return_value, conversion
                );
                return_value *= conversion;
            }

            return Ok(return_value);
        }

        error!("Unable to get internal graph for unit type {}", unit_type);
        Err(ConversionError::new(
            "Unable to get internal graph for unit type",
        ))
    }

    fn get_graph_internal(&self, category: &str) -> Option<&Graph<String, f64>> {
        for graph in &self.graph {
            if graph.id == category {
                return Some(&graph);
            }
        }
        None
    }
}
