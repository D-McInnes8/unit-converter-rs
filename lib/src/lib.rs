use self::parser::{parse_conversion, Unit};

mod graph;
mod parser;
mod persistence;
mod units;

pub trait ConversionStore {
    fn get_default_conversions(&self) -> Result<Vec<UnitConversion>, ()>;
}

pub struct UnitConverter {}

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

    pub fn convert_from_expression(&mut self, input: &str) -> Option<f32> {
        match parse_conversion(&input) {
            Ok(conversion) => {
                println!("{:?}", conversion);
                return self.convert_from_definition(
                    conversion.from,
                    conversion.to,
                    conversion.value,
                );
            }
            Err(_) => {
                eprintln!("Unable to parse expression");
                return None;
            }
        }
    }

    pub fn convert_from_definition(&mut self, from: Unit, to: Unit, value: f32) -> Option<f32> {
        None
    }

    pub fn add_default_conversions(&mut self, store: &impl ConversionStore) -> Result<(), ()> {
        let default_conversions = store.get_default_conversions()?;
        Ok(())
    }
}
