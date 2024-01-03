#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum LengthUnit {
    Millimeters,
    Centimeters,
    Meters,
    Kilometers,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum WeightUnit {
    Milligrams,
    Centigrams,
    Grams,
    Kilograms,
    Tonne,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CapacityUnit {
    Millimeters,
    Centiliters,
    Liters,
    Kiloliters,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Unit {
    Temperature(TemperatureUnit),
    Length(LengthUnit),
    Weight(WeightUnit),
    Capacity(CapacityUnit),
}

pub fn are_unit_types_equal(a: Unit, b: Unit) -> bool {
    std::mem::discriminant(&a) == std::mem::discriminant(&b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units_of_same_type_equal() {
        let a = Unit::Capacity(CapacityUnit::Liters);
        let b = Unit::Capacity(CapacityUnit::Kiloliters);

        assert!(are_unit_types_equal(a, b));
    }

    #[test]
    fn units_of_different_types_equal() {
        let a = Unit::Capacity(CapacityUnit::Liters);
        let b = Unit::Temperature(TemperatureUnit::Celsius);

        assert!(!are_unit_types_equal(a, b));
    }
}
