use log::{debug, info};
use toml::{Table, Value};

use crate::converter::error::ConversionError;
use crate::parser::UnitAbbreviation;
use crate::source::toml::parse_helper::{parse_array, parse_table};

pub struct UnitDefinitionSourceToml {
    path: String,
}

impl UnitDefinitionSourceToml {
    pub fn new(path: &str) -> UnitDefinitionSourceToml {
        UnitDefinitionSourceToml {
            path: path.to_owned(),
        }
    }

    pub fn load(&self) -> Result<Vec<crate::parser::UnitAbbreviation>, ConversionError> {
        info!("Loading unit abbreviations");
        let contents = std::fs::read_to_string(&self.path)?;
        let config = contents.parse::<Table>()?;

        let mut result = vec![];
        for (category, units) in &config {
            for (unit, abbreviations) in parse_table(units)? {
                debug!(
                    "Loading abbreviations {:?} for unit {}",
                    &abbreviations, &unit
                );

                for value in parse_array(abbreviations)? {
                    if let Value::String(abbrev) = value {
                        result.push(UnitAbbreviation {
                            unit: unit.to_owned(),
                            abbrev: abbrev.to_owned(),
                            unit_type: category.to_owned(),
                        });
                    }
                }
            }
        }

        Ok(result)
    }
}
