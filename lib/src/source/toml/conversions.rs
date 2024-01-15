use log::{debug, info};
use toml::{Table, Value};

use crate::converter::UnitConversion;
use crate::source::BaseConversionSource;

pub struct BaseConversionsSourceToml {
    path: String,
    optional: bool,
}

impl BaseConversionsSourceToml {
    pub fn new(path: &str, optional: bool) -> BaseConversionsSourceToml {
        BaseConversionsSourceToml {
            path: path.to_owned(),
            optional: optional,
        }
    }
}

impl BaseConversionSource for BaseConversionsSourceToml {
    fn load(
        &self,
    ) -> Result<Vec<crate::converter::UnitConversion>, crate::converter::error::ConversionError>
    {
        let contents = std::fs::read_to_string(&self.path)?;
        let config = contents.parse::<Table>()?;

        let mut result = vec![];
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
                                result.push(UnitConversion {
                                    value: num,
                                    from: unit_from.to_string(),
                                    to: unit_to.to_string(),
                                    unit_type: category.to_string(),
                                });
                            }
                        }
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

    fn optional(&self) -> bool {
        self.optional
    }
}
