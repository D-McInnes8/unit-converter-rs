use std::error::Error;

use log::{debug, error, info};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{alpha0, alpha1, digit0},
    combinator::{map_res, value},
    error::context,
    number::complete::{double, float},
    sequence::{tuple, Tuple},
    Err, IResult, Parser,
};

use crate::{
    units::{LengthUnit, TemperatureUnit, Unit},
    ConversionError, UnitConversion,
};

#[derive(Debug)]
pub struct UnitAbbreviation {
    pub unit: String,
    pub abbrev: String,
}

pub fn parse_conversion(
    abbreviations: &Vec<UnitAbbreviation>,
    input: &str,
) -> Result<UnitConversion, ConversionError> {
    info!("Attempting to parse expression {}", input,);
    debug!(
        "Parse function has abbreviations for {} units: {:?}",
        &abbreviations.len(),
        &abbreviations
    );

    let result = context(
        "conversion",
        tuple((
            parse_number,
            parse_abbreviation,
            parse_operator,
            parse_abbreviation,
        )),
    )(input);

    match result {
        Ok((_, (value, convert_from, _, convert_to))) => {
            let parsed_convert_from = parse_unit(&abbreviations, convert_from)?;
            debug!(
                "Parsed first unit from {} to {}",
                convert_from, parsed_convert_from
            );

            let parsed_convert_to = parse_unit(&abbreviations, convert_to)?;
            debug!(
                "Parsed second unit from {} to {}",
                convert_to, parsed_convert_to
            );

            return Ok(UnitConversion {
                value: value,
                from: parsed_convert_from,
                to: parsed_convert_to,
            });
        }
        Err(err) => {
            //eprintln!("Parse Error: {}", err);
            error!("Error parsing expression {}", input);
            error!("{}", err);
            return Err(ConversionError::new("Error parsing expression"));
        }
    }
}

fn parse_number(input: &str) -> IResult<&str, f64> {
    context("value", double)(input)
}

fn parse_abbreviation(input: &str) -> IResult<&str, &str> {
    context("unit", alpha1)(input)
}

fn parse_operator(input: &str) -> IResult<&str, &str> {
    alt((tag(" -> "), tag("->"), tag(" to ")))(input)
}

fn parse_unit(units: &Vec<UnitAbbreviation>, input: &str) -> Result<String, ConversionError> {
    let input_lc = input.to_lowercase();
    for unit in units {
        if unit.abbrev == input_lc {
            return Ok(unit.unit.to_string());
        }
    }
    error!("Error parsing {} into a valid unit", input);
    Err(ConversionError::new(&format!(
        "Unable to parse {} into a valid unit",
        input
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct_unit_abbreviations() -> Vec<UnitAbbreviation> {
        vec![
            UnitAbbreviation {
                unit: "Celsius".to_string(),
                abbrev: "C".to_string(),
            },
            UnitAbbreviation {
                unit: "Fahrenheit".to_string(),
                abbrev: "F".to_string(),
            },
        ]
    }

    #[test]
    fn valid_temperature_conversion() {
        let mut input = "20C -> F";
        let abbreviations = construct_unit_abbreviations();

        let expected = UnitConversion {
            value: 20.0,
            from: "Celsius".to_string(),
            to: "Fahrenheit".to_string(),
        };
        let actual = parse_conversion(&abbreviations, &mut input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_unit() {
        let mut input = "20x -> F";
        let abbreviations = construct_unit_abbreviations();

        let actual = parse_conversion(&abbreviations, &mut input);
        assert!(actual.is_err());
    }

    #[test]
    fn invalid_unit_value() {
        let mut input = "C -> F";
        let abbreviations = construct_unit_abbreviations();

        let actual = parse_conversion(&abbreviations, &mut input);
        assert!(actual.is_err());
    }
}
