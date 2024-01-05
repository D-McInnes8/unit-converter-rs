use std::io;

use clap::Parser;
use log::{debug, info};
use unitconvert::converter::builder::UnitConverterBuilder;

use crate::options::CliOptions;

use self::logger::ConsoleLogger;

mod logger;
mod options;

fn main() {
    let cli = CliOptions::parse();
    ConsoleLogger::init(&cli.debug);
    debug!("Cli args: {:?}", cli);

    info!("Building unit converter object");
    let mut converter = UnitConverterBuilder::new()
        .show_debug_messages(true)
        .auto_reverse_conversions(true)
        .add_unit_definitions_toml("Units.toml")
        .add_default_conversions_toml("Base_Conversions.toml")
        .build();

    info!("Waiting for user input");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    return;
                } else {
                    let command = remove_new_line_characters(&input);
                    if command == "exit" {
                        return;
                    }

                    match converter.convert_from_expression(&command) {
                        Ok(new_value) => {
                            println!("{}", new_value);
                        }
                        Err(err) => {
                            eprintln!("{}", err);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        }
    }
}

fn remove_new_line_characters(input: &String) -> &str {
    let result = input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input);
    return result;
}
