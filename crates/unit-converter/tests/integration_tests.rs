use self::common::{setup, setup_test_logger};
use test_case::test_case;

mod common;

#[test_case("2k -> m",           2000.0               ; "kilometers to meters")]
#[test_case("1k -> m",           1000.0               ; "meters to kilometers")]
#[test_case("2.5k -> mi",        1.5534318416245172   ; "kilometers to miles")]
#[test_case("2km -> nmi",        1.0799136069114472   ; "kilometers to nautical miles")]
#[test_case("2.092333nmi -> m",  3875.000716          ; "nautical miles to meters")]
#[test_case("1cm -> km",         0.00001              ; "centermeters to kilometers")]
#[test_case("3.27km -> nm",      3270000000000.0      ; "kilometers to nanometers")]
#[test_case("453406564nm -> km", 0.0004534065640000001; "nanometers to kilometers")]
#[test_case("87pm -> nm",        0.087000000000000001 ; "picometers to nanometers")]
#[test_case("1ly -> km",         9460730472580.0      ; "light-years to kilometers")]
pub fn length_conversion(input: &str, expected: f64) {
    setup_test_logger();
    let mut converter = setup();

    // Run each test case twice to ensure that any caching doesn't alter the result.
    for _ in 0..2 {
        let actual = converter.convert_from_expression(input);

        assert!(actual.is_ok(), "Returned error {:?}", actual.err());
        assert_eq!(expected, actual.unwrap().value);
    }
}

#[test_case("1.079913e9km -> nmi", 583106371.4902809          ; "kilometers to nautical miles")]
#[test_case("9.9999e4km -> m",     99999000.0                 ; "kilometers to meters")]
#[test_case("4.5e-2ly -> nm",      425732871266100000000000.0 ; "light-years to nanometers")]
#[test_case("-7.89e1mi -> m",      -126976.926                ; "miles to meters")]
#[test_case("-5.6e-5nm -> pm",     -0.056                     ; "nanometers to picometers")]
pub fn e_notation(input: &str, expected: f64) {
    let mut converter = setup();

    // Run each test case twice to ensure that any caching doesn't alter the result.
    for _ in 0..2 {
        let actual = converter.convert_from_expression(input);

        assert!(actual.is_ok(), "Returned error {:?}", actual.err());
        assert_eq!(expected, actual.unwrap().value);
    }
}

#[test_case("20C -> F",          68.0                 ; "celsius to fahrenheit")]
#[test_case("20C -> K",          293.15               ; "celsius to kelvin")]
#[test_case("100F -> C",         37.7778              ; "fahrenheit to celsius")]
#[test_case("100F -> K",         310.928              ; "fahrenheit to kelvin")]
#[test_case("300K -> C",         26.85                ; "kelvin to celsius")]
#[test_case("300K -> F",         80.33                ; "kelvin to fahrenheit")]
pub fn temperature_conversion(input: &str, expected: f64) {
    let mut converter = setup();
    let actual = converter.convert_from_expression(input);

    assert!(actual.is_ok(), "Returned error {:?}", actual.err());
    assert_eq!(expected, actual.unwrap().value);
}
