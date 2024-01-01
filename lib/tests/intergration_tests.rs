use unitconvert::UnitConverter;

#[test]
pub fn no_default_conversions() {
    let mut converter = UnitConverter::new();
    let actual = converter.convert_from_expression("20F -> C");
    assert_eq!(actual, None);
}

#[test]
pub fn celsius_to_fahrenheit() {
    let mut converter = UnitConverter::new();
    let actual = converter.convert_from_expression("20C -> F");
    assert_eq!(actual, Some(68.0));
}

#[test]
pub fn fahrenheit_to_celsius() {
    let mut converter = UnitConverter::new();
    let actual = converter.convert_from_expression("100F -> C");
    assert_eq!(actual, Some(37.7778));
}
