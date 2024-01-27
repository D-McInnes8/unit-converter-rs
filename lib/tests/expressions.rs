use test_case::test_case;
use unitconvert::expressions::expression::eval;

#[test_case("10 + 5 - 2",                    13.0          ; "subtraction_operator")]
#[test_case("10 + 5 * 2",                    20.0          ; "multiplication_operator")]
#[test_case("10 + 5 / 2",                    12.5          ; "division_operator")]
#[test_case("5 * (2.5 + 7.3)",               49.0          ; "float_with_parenthesis")]
#[test_case("3 + 4 × 2 ÷ ( 1 − 5 ) ^ 2 ^ 3", 3.00012207031 ; "complex_equation")]
#[test_case("cos (π)",                       -1.0          ; "simple_function")]
#[test_case("max (2, 3)",                    3.0           ; "simple_max_function")]
#[test_case("sin ( max ( 2, 3 ) ÷ 3 × π )",  0.0           ; "complex_equation_with_functions")]
#[test_case("-2 + 5",                        3.0           ; "single_negative_number")]
#[test_case("-2 - -5",                       3.0           ; "multiple_negative_numbers")]
#[test_case("5 + 5 % 10",                    10.0          ; "modulus_operator")]
#[test_case("2^5 * 10",                      320.0         ; "exponential_operator")]
#[test_case("10 + 2^5",                      42.0          ; "exponential_operator_with_addition")]
#[test_case("2^(10 - 2) % 10",               6.0           ; "exponential_operator_with_parenthesis")]
#[test_case("(0.3456 + 0.766) * 120.763",    134.2401508   ; "decimal_numbers")]
pub fn expression_no_parameters(exp: &str, expected: f64) {
    let actual = eval(exp);
    assert!(actual.is_ok(), "Returned error {:?}", actual.err());
    assert_eq!(expected, actual.unwrap());
}

#[test_case("10 45"      ; "no_operator")]
#[test_case("+ 10"       ; "missing_number")]
#[test_case("2 * (1 + 5" ; "mismatched_parenthesis right")]
#[test_case("1 + 5) * 2" ; "mismatched_parenthesis_left")]
pub fn invalid_expression(exp: &str) {
    let actual = eval(exp);
    assert!(actual.is_err(), "Should be err. Returned {:?}", actual);
}
