use unitconvert::persistence::in_memory::InMemoryConversionStore;
use unitconvert::units::{LengthUnit, Unit};
use unitconvert::UnitConverter;

#[test]
fn convert_kilometers_to_meters() {
    let mut converter = UnitConverter::new();
    let mut store = InMemoryConversionStore::new();
    store.insert(
        Unit::Length(LengthUnit::Meters),
        Unit::Length(LengthUnit::Kilometers),
        0.001,
    );
    converter.add_default_conversions(&store);

    let actual = converter.convert_from_expression("1k -> m");
    assert_eq!(actual.ok(), Some(1000.0));
}
