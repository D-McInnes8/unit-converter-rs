#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum LengthUnit {
    // Metric
    Millimeters,
    Centimeters,
    Meters,
    Kilometers,

    // Imperial
    Inch,
    Hand,
    Feet,
    Yard,
    Mile,
    League,

    // Maritime
    Fathom,
    Cable,
    NauticalMile,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum WeightUnit {
    // Metric
    Milligrams,
    Centigrams,
    Grams,
    Kilograms,
    Tonne,

    // Imperial
    Grain,
    Ounce,
    Quarter,
    Stone,
    Pound,
    Ton,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CapacityUnit {
    // Metric
    Millimeters,
    Centiliters,
    Liters,
    Kiloliters,

    // Imperial
    FluidOunce,
    Gill,
    Pint,
    Quart,
    Gallon,
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
