use std::collections::HashSet;

use log::{debug, info};

use crate::converter::Conversion;
use crate::graph::Graph;
use crate::parser::UnitAbbreviation;

use super::error::ConversionError;
use super::{UnitConversion, UnitConverter};

pub struct UnitConverterBuilder {
    unit_types: HashSet<String>,
    conversions: Vec<UnitConversion>,
    abbreviations: Vec<UnitAbbreviation>,
    auto_reverse: bool,
    cache: bool,
}

impl Default for UnitConverterBuilder {
    fn default() -> Self {
        UnitConverterBuilder {
            unit_types: HashSet::new(),
            conversions: vec![],
            abbreviations: vec![],
            auto_reverse: false,
            cache: true,
        }
    }
}

impl UnitConverterBuilder {
    pub fn new() -> UnitConverterBuilder {
        UnitConverterBuilder::default()
    }

    pub fn reverse_base_conversions(mut self, include: bool) -> UnitConverterBuilder {
        self.auto_reverse = include;
        self
    }

    pub fn cache_results(mut self, cache: bool) -> UnitConverterBuilder {
        self.cache = cache;
        self
    }

    pub fn add_base_conversions(
        mut self,
        mut conversions: Vec<UnitConversion>,
    ) -> UnitConverterBuilder {
        self.conversions.append(&mut conversions);
        self
    }

    pub fn add_unit_definitions(
        mut self,
        mut units: Vec<UnitAbbreviation>,
    ) -> UnitConverterBuilder {
        for unit in &units {
            if !self.unit_types.contains(&unit.unit_type) {
                self.unit_types.insert(unit.unit_type.to_owned());
            }
        }
        self.abbreviations.append(&mut units);
        self
    }

    pub fn build(self) -> Result<UnitConverter, ConversionError> {
        // Populate graph
        let mut graphs = vec![];
        for unit_type in &self.unit_types {
            let mut graph = Graph::new(unit_type.to_owned());
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
                _ = graph.add_edge(n0, n1, Conversion::Multiplier(conversion.value));

                if self.auto_reverse {
                    let reversed = 1.0 / conversion.value;
                    debug!(
                        "Adding reversed edge to '{}' graph for {} -> {} (x *= {})",
                        unit_type, &conversion.to, &conversion.from, reversed
                    );
                    _ = graph.add_edge(n1, n0, Conversion::Multiplier(reversed));
                }

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
        Ok(UnitConverter::new(graphs, self.abbreviations, self.cache))
    }
}
