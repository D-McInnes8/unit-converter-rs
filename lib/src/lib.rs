use core::fmt;
use std::error::Error;

use toml::{Table, Value};

use self::graph::Graph;
use self::parser::parse_conversion;
use self::units::Unit;

mod graph;
mod parser;
pub mod persistence;
pub mod units;

pub trait ConversionStore {
    fn get_default_conversions(&self) -> Result<Vec<UnitConversion>, ()>;
}

pub struct UnitConverter {
    graph: Graph<Unit, f32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnitConversion {
    pub value: f32,
    pub from: Unit,
    pub to: Unit,
}

impl UnitConverter {
    pub fn builder() -> UnitConverterBuilder {
        UnitConverterBuilder::new()
    }

    pub fn new() -> UnitConverter {
        UnitConverter {
            graph: Graph::new(),
        }
    }

    pub fn new2(graph: Graph<Unit, f32>) -> UnitConverter {
        UnitConverter { graph: graph }
    }

    pub fn convert_from_expression(&mut self, input: &str) -> Result<f32, ConversionError> {
        match parse_conversion(&input) {
            Ok(conversion) => {
                println!("{:?}", conversion);
                return self.convert_from_definition(
                    conversion.from,
                    conversion.to,
                    conversion.value,
                );
            }
            Err(err) => {
                return Err(ConversionError::new());
            }
        }
    }

    pub fn convert_from_definition(
        &mut self,
        from: Unit,
        to: Unit,
        value: f32,
    ) -> Result<f32, ConversionError> {
        let n0 = self.graph.get_node_index(from).unwrap();
        let n1 = self.graph.get_node_index(to).unwrap();
        let shortest_path = self.graph.shortest_path(n0, n1);

        let mut return_value = value;
        for (unit, conversion) in shortest_path {
            println!(
                "Applying value * {} ({:?}). Expression is {} *= {}",
                conversion, unit, return_value, conversion
            );
            return_value *= conversion;
        }

        Ok(return_value)

        //Err(ConversionError::new())
    }

    pub fn add_default_conversions(&mut self, store: &impl ConversionStore) -> Result<(), ()> {
        let default_conversions = store.get_default_conversions()?;

        for conversion in default_conversions {
            println!(
                "Adding conversion of {:?} -> {:?} ({})",
                conversion.from, conversion.to, conversion.value
            );
            let reverse = 1.0 / conversion.value;
            let n0 = self.graph.add_node(conversion.from);
            let n1 = self.graph.add_node(conversion.to);
            _ = self.graph.add_edge(n0, n1, conversion.value);
            _ = self.graph.add_edge(n1, n0, reverse);
            println!(
                "Adding conversion of {:?} -> {:?} ({})",
                conversion.to, conversion.from, reverse
            );
        }

        Ok(())
    }
}

pub struct UnitConverterBuilder {
    graph: Graph<Unit, f32>,
    include_reversed_values: bool,
}

impl UnitConverterBuilder {
    pub fn new() -> UnitConverterBuilder {
        UnitConverterBuilder {
            graph: Graph::new(),
            include_reversed_values: false,
        }
    }

    pub fn include_reversed_conversion(mut self, include: bool) -> UnitConverterBuilder {
        self.include_reversed_values = include;
        self
    }

    pub fn add_batch(mut self, definitions: Vec<UnitConversion>) -> UnitConverterBuilder {
        self
    }

    pub fn add_file(self) -> UnitConverterBuilder {
        self
    }

    pub fn add_toml_conversions(self, file_path: &str) -> UnitConverterBuilder {
        let contents =
            std::fs::read_to_string(file_path).expect("Unable to load Toml base conversions.");
        let config = contents.parse::<Table>().unwrap();

        println!("Configuration:");
        for (category, list) in config {
            if let Value::Table(units) = list {
                for (unit_from, conversions) in units {
                    if let Value::Table(b) = conversions {
                        for (unit_to, value) in b {
                            println!("[{}] {} -> {}: {}", category, unit_from, unit_to, value);
                        }
                    }
                    //println!("[{}] {} -> {}: {}", category, "", unit, value);
                }
            }
        }

        /*for (key, value2) in value {
            println!("{}: {:?}", key, value2);

            if let Value::Table(table) = value2 {
                for (key1, value3) in table {
                    println!("{}: {:?}", key1, value3);
                }
            }
        }*/

        self
    }

    pub fn add_conversion(mut self, from: Unit, to: Unit, value: f32) -> UnitConverterBuilder {
        let n0 = self.graph.add_node(from);
        let n1 = self.graph.add_node(to);

        _ = self.graph.add_edge(n0, n1, value);
        if self.include_reversed_values {
            let reversed = 1.0 / value;
            _ = self.graph.add_edge(n1, n0, reversed);
        }

        self
    }

    pub fn build(self) -> UnitConverter {
        UnitConverter::new2(self.graph)
    }
}

#[derive(Debug)]
pub struct ConversionError {
    source: Option<Box<dyn Error>>,
}

impl ConversionError {
    fn new() -> ConversionError {
        ConversionError { source: None }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error executing conversion")
    }
}

impl Error for ConversionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}
