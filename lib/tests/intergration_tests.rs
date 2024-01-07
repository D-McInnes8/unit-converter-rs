use self::common::setup;
use test_case::test_case;

mod common;

#[test_case("2k -> m",           2000.0               ; "kilometers to meters")]
#[test_case("1k -> m",           1000.0               ; "meters to kilometers")]
#[test_case("2.5k -> mi",        1.553431841624517    ; "kilometers to miles")]
#[test_case("2km -> nmi",        1.0799136069114472   ; "kilometers to nautical miles")]
#[test_case("2.092333nmi -> m",  3875.000716          ; "nautical miles to meters")]
#[test_case("1cm -> km",         0.00001              ; "centermeters to kilometers")]
#[test_case("3.27km -> nm",      3270000000000.0      ; "kilometers to nanometers")]
#[test_case("453406564nm -> km", 0.000453406564       ; "nanometers to kilometers")]
#[test_case("87pm -> nm",        0.087000000000000001 ; "picometers to nanometers")]
pub fn valid_conversion(input: &str, expected: f64) {
    let mut converter = setup();
    let actual = converter.convert_from_expression(input);

    assert!(actual.is_ok(), "Returned error {:?}", actual.err());
    assert_eq!(expected, actual.unwrap());
}

/*#[test]
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
