use std::collections::HashSet;

use log::{debug, error, info};
use toml::{Table, Value};

use self::converter::error::ConversionError;
use self::graph::Graph;
use self::parser::{parse_conversion, UnitAbbreviation};

pub mod converter;
mod graph;
mod parser;
pub mod units;

pub trait ConversionStore {
    fn get_default_conversions(&self) -> Result<Vec<UnitConversion>, ()>;
}

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

pub struct UnitConverterBuilder {
    unit_types: HashSet<String>,
    conversions: Vec<UnitConversion>,
    abbreviations: Vec<UnitAbbreviation>,
    include_reversed_values: bool,
    show_debug_messages: bool,
}

impl UnitConverterBuilder {
    pub fn new() -> UnitConverterBuilder {
        UnitConverterBuilder {
            unit_types: HashSet::new(),
            conversions: vec![],
            abbreviations: vec![],
            include_reversed_values: false,
            show_debug_messages: false,
        }
    }

    pub fn include_reversed_conversion(mut self, include: bool) -> UnitConverterBuilder {
        self.include_reversed_values = include;
        self
    }

    pub fn show_debug_messages(mut self, show: bool) -> UnitConverterBuilder {
        self.show_debug_messages = show;
        self
    }

    pub fn add_batch(mut self, definitions: Vec<UnitConversion>) -> UnitConverterBuilder {
        self
    }

    pub fn add_file(self) -> UnitConverterBuilder {
        self
    }

    pub fn add_toml_units(mut self, file_path: &str) -> UnitConverterBuilder {
        let contents = std::fs::read_to_string(file_path)
            .expect("Unable to load unit abbreviations from toml file.");
        let config = contents.parse::<Table>().unwrap();

        info!("Loading unit abbreviations");
        for (category, units_table) in config {
            if let Value::Table(units) = units_table {
                for (unit, abbreviations_array) in &units {
                    debug!(
                        "Loading abbreviations {:?} for unit {}",
                        &abbreviations_array, &unit
                    );

                    if let Value::Array(abbreviations) = abbreviations_array {
                        for value in abbreviations {
                            if let Value::String(abbrev) = value {
                                self.abbreviations.push(UnitAbbreviation {
                                    unit: unit.to_string(),
                                    abbrev: abbrev.to_string(),
                                    unit_type: category.to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }

        self
    }

    pub fn add_toml_conversions(mut self, file_path: &str) -> UnitConverterBuilder {
        let contents =
            std::fs::read_to_string(file_path).expect("Unable to load Toml base conversions.");
        let config = contents.parse::<Table>().unwrap();
        let initial_count = self.conversions.len();

        for (category, list) in config {
            self.unit_types.insert(category.clone());
            if let Value::Table(units) = list {
                for (unit_from, conversions) in units {
                    if let Value::Table(b) = conversions {
                        for (unit_to, value) in b {
                            debug!(
                                "Imported Base Conversion: [{}] {} -> {}: {}",
                                category, unit_from, unit_to, value
                            );

                            let f_value = match value {
                                Value::Float(f) => Some(f),
                                Value::Integer(i) => Some(i as f64),
                                _ => None,
                            };

                            if let Some(num) = f_value {
                                //self.add_conversion(&unit_from, &unit_to, num);
                                self.conversions.push(UnitConversion {
                                    value: num,
                                    from: unit_from.to_string(),
                                    to: unit_to.to_string(),
                                    unit_type: category.to_string(),
                                });
                                if self.include_reversed_values {
                                    let reversed = 1.0 / num;
                                    self.conversions.push(UnitConversion {
                                        value: reversed,
                                        from: unit_to.to_string(),
                                        to: unit_from.to_string(),
                                        unit_type: category.to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        info!(
            "Imported {} default unit conversions from {}",
            self.conversions.len() - initial_count,
            file_path
        );

        self
    }

    pub fn add_conversion(
        mut self,
        unit_type: &str,
        from: &str,
        to: &str,
        value: f64,
    ) -> UnitConverterBuilder {
        self.conversions.push(UnitConversion {
            value: value,
            from: from.to_string(),
            to: to.to_string(),
            unit_type: unit_type.to_string(),
        });

        if self.include_reversed_values {
            let reversed = 1.0 / value;
            self.conversions.push(UnitConversion {
                value: reversed,
                from: to.to_string(),
                to: from.to_string(),
                unit_type: unit_type.to_string(),
            });
        }

        self
    }

    pub fn build(self) -> UnitConverter {
        let mut graphs = vec![];

        for unit_type in &self.unit_types {
            let mut graph = Graph::new(unit_type.to_string());
            let mut count = 0;

            for conversion in &self.conversions {
                if conversion.unit_type != *unit_type {
                    continue;
                }

                debug!(
                    "Adding edge to '{}' graph for default conversion {} -> {} (x *= {})",
                    unit_type, &conversion.from, &conversion.to, &conversion.value
                );
                let n0 = graph.add_node(conversion.from.clone());
                let n1 = graph.add_node(conversion.to.clone());
                _ = graph.add_edge(n0, n1, conversion.value);
                count += 1;
            }
            info!(
                "Populated graph for type {} with {} default conversions",
                &unit_type, count
            );
            graphs.push(graph);
        }

        info!(
            "Finished building unit converter object. Contains graphs for {} unit type(s) and definitions for {} unit(s)",
            graphs.len(), &self.abbreviations.len()
        );
        UnitConverter::new(graphs, self.abbreviations)
    }
}
