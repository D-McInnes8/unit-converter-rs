use crate::units::Unit;
use crate::{ConversionStore, UnitConversion};

pub struct InMemoryConversionStore {
    default_conversions: Vec<UnitConversion>,
}

impl InMemoryConversionStore {
    pub fn new() -> InMemoryConversionStore {
        InMemoryConversionStore {
            default_conversions: Vec::new(),
        }
    }

    pub fn insert(&mut self, a: Unit, b: Unit, value: f32) {
        self.default_conversions.push(UnitConversion {
            from: b,
            to: a,
            value: value,
        })
    }

    pub fn add(&mut self, conversion: UnitConversion) {
        self.default_conversions.push(conversion);
    }

    pub fn clear(&mut self) {
        self.default_conversions.clear();
    }
}

impl ConversionStore for InMemoryConversionStore {
    fn get_default_conversions(&self) -> Result<Vec<UnitConversion>, ()> {
        Ok(self.default_conversions.clone())
    }
}
