use crate::converter::error::ConversionError;
use crate::converter::UnitConversion;
use crate::parser::UnitAbbreviation;

pub mod toml;

pub trait BaseConversionSource {
    fn load(&self) -> Result<Vec<UnitConversion>, ConversionError>;
    fn optional(&self) -> bool;
}

pub trait UnitDefitionSource {
    fn load(&self) -> Result<Vec<UnitAbbreviation>, ConversionError>;
    fn optional(&self) -> bool;
}
