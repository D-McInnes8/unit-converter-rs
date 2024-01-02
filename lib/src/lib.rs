use core::fmt;
use std::error::Error;

use nom::Err;

use self::parser::parse_conversion;
use self::units::Unit;

mod graph;
mod parser;
mod persistence;
mod units;

pub trait ConversionStore {
    fn get_default_conversions(&self) -> Result<Vec<UnitConversion>, ()>;
}

pub struct UnitConverter;

#[derive(Debug, PartialEq, Clone)]
pub struct UnitConversion {
    pub value: f32,
    pub from: Unit,
    pub to: Unit,
}

impl UnitConverter {
    pub fn new() -> UnitConverter {
        UnitConverter {}
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
        Err(ConversionError::new())
    }

    pub fn add_default_conversions(&mut self, store: &impl ConversionStore) -> Result<(), ()> {
        let default_conversions = store.get_default_conversions()?;
        Ok(())
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
