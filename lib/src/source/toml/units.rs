use std::io::Error;

use log::{debug, info};
use toml::map::Map;
use toml::{Table, Value};

use crate::converter::error::ConversionError;
use crate::parser::UnitAbbreviation;
use crate::source::UnitDefitionSource;

pub struct UnitDefinitionSourceToml {
    path: String,
    optional: bool,
}

impl From<Error> for ConversionError {
    fn from(value: Error) -> Self {
        ConversionError::new(&value.to_string())
    }
}

impl From<toml::de::Error> for ConversionError {
    fn from(value: toml::de::Error) -> Self {
        ConversionError::new(value.message())
    }
}

impl UnitDefitionSource for UnitDefinitionSourceToml {
    fn load(&self) -> Result<Vec<crate::parser::UnitAbbreviation>, ConversionError> {
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

    fn optional(&self) -> bool {
        self.optional
    }
}

fn parse_table(value: &Value) -> Result<&Map<String, Value>, ConversionError> {
    if let Value::Table(tbl) = value {
        return Ok(tbl);
    }
    Err(ConversionError::default())
}

fn parse_array(value: &Value) -> Result<&Vec<Value>, ConversionError> {
    if let Value::Array(vec) = value {
        return Ok(vec);
    }
    Err(ConversionError::default())
}
