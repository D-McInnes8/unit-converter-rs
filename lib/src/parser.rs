use std::error::Error;

use log::error;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{alpha0, digit0},
    combinator::{map_res, value},
    error::context,
    number::complete::float,
    sequence::{tuple, Tuple},
    IResult, Parser,
};

use crate::{
    units::{LengthUnit, TemperatureUnit, Unit},
    ConversionError, UnitConversion,
};

pub fn parse_conversion(input: &str) -> Result<UnitConversion, ConversionError> {
    let result = context(
        "conversion",
        tuple((parse_number, parse_unit, parse_operator, parse_unit)),
    )(input);

    match result {
        Ok((_, (value, convert_from, _, convert_to))) => {
            return Ok(UnitConversion {
                value: value as f64,
                from: convert_from.to_string(),
                to: convert_to.to_string(),
            })
        }
        Err(err) => {
            //eprintln!("Parse Error: {}", err);
            error!("Error parsing expression {}", input);
            error!("{}", err);
            return Err(ConversionError::new());
        }
    }
}

fn parse_number(input: &str) -> IResult<&str, f32> {
    context("value", float)(input)
}

fn parse_unit(input: &str) -> IResult<&str, &str> {
    alt((
        value("Miles", tag("mi")),
        value("Celsius", tag("C")),
        value("Fahrenheit", tag("F")),
        value("Kelvin", tag("K")),
        value("Kilometers", tag("k")),
        value("Meters", tag("m")),
    ))(input)
}

fn parse_operator(input: &str) -> IResult<&str, &str> {
    alt((tag(" -> "), tag("->"), tag(" to ")))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_temperature_conversion() {
        let mut input = "20C -> F";
        let expected = UnitConversion {
            value: 20.0,
            from: "Celsius".to_string(),
            to: "Fahrenheit".to_string(),
        };
        let actual = parse_conversion(&mut input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_unit() {
        let mut input = "20x -> F";
        let actual = parse_conversion(&mut input);
        assert!(actual.is_err());
    }

    #[test]
    fn invalid_unit_value() {
        let mut input = "C -> F";
        let actual = parse_conversion(&mut input);
        assert!(actual.is_err());
    }
}
