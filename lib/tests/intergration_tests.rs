use self::common::setup;
use test_case::test_case;

mod common;

#[test_case("2k -> m",    2000.0             ; "kilometers to meters")]
#[test_case("1k -> m",    1000.0             ; "meters to kilometers")]
#[test_case("2.5k -> mi", 1.553431841624517  ; "kilometers to miles")]
#[test_case("2km -> nmi", 1.0799136069114472 ; "kilometers to nautical miles")]
pub fn valid_conversion(input: &str, expected: f64) {
    let mut converter = setup();
    let actual = converter.convert_from_expression(input);

    assert!(actual.is_ok(), "Returned error {:?}", actual.err());
    assert_eq!(expected, actual.unwrap());
}

/*#[test]
pub fn kilometers_to_meters() {
    let mut converter = setup();
    let actual = converter.convert_from_expression("2k -> m");
    assert_eq!(actual.ok(), Some(2000.0))
}*/

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
