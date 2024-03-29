use std::{io, process};

use clap::Parser;
use console::{style, Color};
use dialoguer::Input;
use log::{debug, info};
use unitconvert::converter::builder::UnitConverterBuilder;
use unitconvert::converter::error::ConversionError;
use unitconvert::converter::UnitConverter;
use unitconvert::source::toml::conversions::BaseConversionsSourceToml;
use unitconvert::source::toml::units::UnitDefinitionSourceToml;

use crate::input::{generate_input_theme, InputHistory};
use crate::options::CliOptions;

use self::logger::ConsoleLogger;

mod input;
mod logger;
mod options;

fn main() {
    let cli = CliOptions::parse();
    ConsoleLogger::init(&cli.debug);
    debug!("Cli args: {:?}", cli);

    info!("Building unit converter object");
    match build_converter() {
        Ok(mut converter) => {
            if cli.interactive {
                let mut history = InputHistory::default();
                let theme = generate_input_theme();
                loop {
                    if let Ok(cmd) = Input::<String>::with_theme(&theme)
                        //.with_prompt(" > ")
                        .history_with(&mut history)
                        .interact_text()
                    {
                        process_cmd(&mut converter, &cmd);
                    }
                }
            } else {
                info!("Waiting for user input");
                loop {
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(len) => {
                            if len == 0 {
                                return;
                            } else {
                                let command = remove_new_line_characters(&input);
                                process_cmd(&mut converter, command);
                            }
                        }
                        Err(error) => {
                            eprintln!("error: {}", error);
                            return;
                        }
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("error initializing: {}", error);
        }
    }
}

fn build_converter() -> Result<UnitConverter, ConversionError> {
    let conversions = BaseConversionsSourceToml::new("Base_Conversions.toml").load()?;
    let units = UnitDefinitionSourceToml::new("Units.toml").load()?;

    UnitConverterBuilder::new()
        .reverse_base_conversions(true)
        .cache_results(true)
        .add_unit_definitions(units)
        .add_base_conversions(conversions)
        .build()
}

fn process_cmd(converter: &mut UnitConverter, cmd: &str) {
    if cmd == "exit" {
        process::exit(0);
    } else if cmd == "units" {
        display_converter_units(converter);
    } else if cmd == "help" {
        show_help_text();
    } else {
        if cmd.chars().all(|x| x.is_alphabetic()) {
            match converter.unit_info(cmd) {
                Ok(u) => println!("{} ({})", u.unit, u.unit_type),
                Err(_) => eprintln!(
                    "{} Unknown command {}",
                    style(format!("{: <5}", "ERROR")).fg(Color::Red).bold(),
                    style(cmd).italic()
                ),
            };
            return;
        }

        match converter.convert_from_expression(cmd) {
            Ok(result) => {
                if result.value > 99999.0 || result.value < 0.00009 {
                    println!(
                        "{:e} {}",
                        style(result.value).fg(console::Color::White).bold(),
                        result.to.to_lowercase()
                    )
                } else {
                    println!(
                        "{} {}",
                        style(result.value).fg(console::Color::White).bold(),
                        result.to.to_lowercase()
                    )
                }
            }
            Err(err) => eprintln!(
                "{} {}",
                style(format!("{: <5}", "ERROR")).fg(Color::Red).bold(),
                err
            ),
        }
    }
}

fn show_help_text() {}

fn display_converter_units(converter: &UnitConverter) {
    let units = converter.units();
    println!("{: <20} {}", style("Unit").bold(), style("Type").bold());
    for unit in units {
        println!(
            "{: <20} {} {}",
            style(&unit.unit).italic(),
            unit.unit_type,
            unit.abbrev
        );
    }
}

fn remove_new_line_characters(input: &String) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix('\n'))
        .unwrap_or(input)
}
