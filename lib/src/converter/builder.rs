use std::collections::HashSet;

use log::{debug, info};
use toml::{Table, Value};

use crate::graph::Graph;
use crate::parser::UnitAbbreviation;

use super::{UnitConversion, UnitConverter};

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
