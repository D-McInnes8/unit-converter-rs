use log::{debug, info};
use toml::{Table, Value};

use crate::source::toml::parse_helper::parse_table;
use crate::ConversionDefinition;
use crate::ConversionValueDefinition;

pub struct BaseConversionsSourceToml {
    path: String,
}

impl BaseConversionsSourceToml {
    pub fn new(path: &str) -> BaseConversionsSourceToml {
        BaseConversionsSourceToml {
            path: path.to_owned(),
        }
    }

    pub fn load(
        &self,
    ) -> Result<Vec<ConversionDefinition>, crate::converter::error::ConversionError> {
        let contents = std::fs::read_to_string(&self.path)?;
        let config = contents.parse::<Table>()?;

        let mut result = vec![];
        for (category, units) in &config {
            for (unit_from, conversions) in parse_table(units)? {
                for (unit_to, value) in parse_table(conversions)? {
                    debug!(
                        "Imported Base Conversion: [{}] {} -> {}: {}",
                        category, unit_from, unit_to, value
                    );

                    if let Some(c) = match_definition_val(value) {
                        result.push(ConversionDefinition {
                            val: c,
                            from: unit_from.to_owned(),
                            to: unit_to.to_owned(),
                            category: category.to_owned(),
                        });
                    }
                }
            }
        }

        info!(
            "Imported {} default unit conversions from {}",
            result.len(),
            &self.path
        );
        Ok(result)
    }
}

fn match_definition_val(val: &Value) -> Option<ConversionValueDefinition> {
    match val {
        Value::Float(f) => Some(ConversionValueDefinition::Multiplier(*f)),
        Value::Integer(i) => Some(ConversionValueDefinition::Multiplier(*i as f64)),
        Value::String(s) => Some(ConversionValueDefinition::Expression(s.to_owned())),
        _ => None,
    }
}
