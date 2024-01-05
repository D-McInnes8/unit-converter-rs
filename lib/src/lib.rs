use core::fmt;
use std::error::Error;

use log::{debug, info, trace};
use toml::{Table, Value};

use self::graph::Graph;
use self::parser::{parse_conversion, UnitAbbreviation};

mod graph;
mod parser;
pub mod units;

pub trait ConversionStore {
    fn get_default_conversions(&self) -> Result<Vec<UnitConversion>, ()>;
}

pub struct UnitConverter {
    graph: Graph<String, f64>,
    abbreviations: Vec<UnitAbbreviation>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnitConversion {
    pub value: f64,
    pub from: String,
    pub to: String,
}

impl UnitConverter {
    pub fn builder() -> UnitConverterBuilder {
        UnitConverterBuilder::new()
    }

    pub fn new() -> UnitConverter {
        UnitConverter {
            graph: Graph::new(),
            abbreviations: vec![],
        }
    }

    pub fn new2(graph: Graph<String, f64>, abbreviations: Vec<UnitAbbreviation>) -> UnitConverter {
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
        from: &str,
        to: &str,
        value: f64,
    ) -> Result<f64, ConversionError> {
        let n0 = self.graph.get_node_index(from.to_string()).unwrap();
        let n1 = self.graph.get_node_index(to.to_string()).unwrap();
        let shortest_path = self.graph.shortest_path(n0, n1);

        let mut return_value = value;
        for (unit, conversion) in shortest_path {
            info!(
                "Converting value to {}. Expresion is {} *= {}",
                unit, return_value, conversion
            );
            return_value *= conversion;
        }

        Ok(return_value)
    }
}

pub struct UnitConverterBuilder {
    conversions: Vec<UnitConversion>,
    abbreviations: Vec<UnitAbbreviation>,
    include_reversed_values: bool,
    show_debug_messages: bool,
}

impl UnitConverterBuilder {
    pub fn new() -> UnitConverterBuilder {
        UnitConverterBuilder {
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

        for (category, units_table) in config {
            if category == "abbreviations" {
                info!("Loading unit abbreviations");
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
                                    });
                                }
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
                                });
                                if self.include_reversed_values {
                                    let reversed = 1.0 / num;
                                    self.conversions.push(UnitConversion {
                                        value: reversed,
                                        from: unit_to.to_string(),
                                        to: unit_from.to_string(),
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

    pub fn add_conversion(mut self, from: &str, to: &str, value: f64) -> UnitConverterBuilder {
        self.conversions.push(UnitConversion {
            value: value,
            from: from.to_string(),
            to: to.to_string(),
        });

        if self.include_reversed_values {
            let reversed = 1.0 / value;
            self.conversions.push(UnitConversion {
                value: reversed,
                from: to.to_string(),
                to: from.to_string(),
            });
        }

        self
    }

    pub fn build(self) -> UnitConverter {
        let mut graph = Graph::new();

        info!(
            "Populating graph with {} default unit conversions",
            &self.conversions.len()
        );
        for conversion in &self.conversions {
            trace!(
                "{} -> {}: {}",
                &conversion.from,
                &conversion.to,
                &conversion.value
            );
            info!(
                "Adding edge to graph for default conversion {} -> {} (x *= {})",
                &conversion.from, &conversion.to, &conversion.value
            );
            let n0 = graph.add_node(conversion.from.clone());
            let n1 = graph.add_node(conversion.to.clone());
            _ = graph.add_edge(n0, n1, conversion.value);
        }

        info!(
            "Finished building unit converter object. Object contains abbreviations for {} units",
            &self.abbreviations.len()
        );
        UnitConverter::new2(graph, self.abbreviations)
    }
}

#[derive(Debug)]
pub struct ConversionError {
    source: Option<Box<dyn Error>>,
    message: Option<String>,
}

impl ConversionError {
    fn default() -> ConversionError {
        ConversionError {
            source: None,
            message: None,
        }
    }

    fn new(message: &str) -> ConversionError {
        ConversionError {
            source: None,
            message: Some(message.to_string()),
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source) = &self.source {
            return write!(f, "{}", source.to_string());
        }

        let error_message = match &self.message {
            Some(err) => err,
            None => "Error executing conversion",
        };
        write!(f, "{}", error_message)
    }
}

impl Error for ConversionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}
