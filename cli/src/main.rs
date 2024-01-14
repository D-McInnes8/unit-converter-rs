use std::{io, process};

use clap::Parser;
use console::{style, Color};
use dialoguer::Input;
use log::{debug, info};
use unitconvert::converter::builder::UnitConverterBuilder;
use unitconvert::converter::UnitConverter;

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
    let builder_result = UnitConverterBuilder::new()
        .show_debug_messages(true)
        .auto_reverse_conversions(true)
        .add_unit_definitions_toml("Units.toml")
        .add_default_conversions_toml("Base_Conversions.toml")
        .build();

    if let Ok(mut converter) = builder_result {
        if cli.interactive == true {
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
}

fn process_cmd(converter: &mut UnitConverter, cmd: &str) {
    if cmd == "exit" {
        process::exit(0);
    } else if cmd == "units" {
        display_converter_units(&converter);
    } else if cmd == "help" {
        show_help_text();
    } else {
        match converter.convert_from_expression(cmd) {
            Ok(result) => {
                println!("{}", style(result).fg(console::Color::White).bold())
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
    let result = input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input);
    return result;
}
