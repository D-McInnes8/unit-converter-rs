use toml::map::Map;
use toml::Value;

use crate::converter::error::ConversionError;

pub fn parse_table(value: &Value) -> Result<&Map<String, Value>, ConversionError> {
    if let Value::Table(tbl) = value {
        return Ok(tbl);
    }
    Err(ConversionError::default())
}

pub fn parse_array(value: &Value) -> Result<&Vec<Value>, ConversionError> {
    if let Value::Array(vec) = value {
        return Ok(vec);
    }
    Err(ConversionError::default())
}
