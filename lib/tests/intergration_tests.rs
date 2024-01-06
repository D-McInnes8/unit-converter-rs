use self::common::setup;

mod common;

#[test]
pub fn kilometers_to_meters() {
    let mut converter = setup();
    let actual = converter.convert_from_expression("2k -> m");
    assert_eq!(actual.ok(), Some(2000.0))
}

/*#[test]
pub fn no_default_conversions() {
    let mut converter = UnitConverter::new();
    let mut store = InMemoryConversionStore::new();
    converter.add_default_conversions(&store);

    let actual = converter.convert_from_expression("20F -> C");
    assert_eq!(actual.ok(), None);
}

#[test]
pub fn celsius_to_fahrenheit() {
    let mut converter = UnitConverter::new();
    let actual = converter.convert_from_expression("20C -> F");
    assert_eq!(actual.ok(), Some(68.0));
}

#[test]
pub fn fahrenheit_to_celsius() {
    let mut converter = UnitConverter::new();
    let actual = converter.convert_from_expression("100F -> C");
    assert_eq!(actual.ok(), Some(37.7778));
}*/
