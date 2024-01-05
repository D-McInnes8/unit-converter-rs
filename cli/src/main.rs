use std::io;

use log::info;
use unitconvert::converter::builder::UnitConverterBuilder;

use self::logger::ConsoleLogger;

mod logger;

fn main() {
    ConsoleLogger::init();

    info!("Building unit converter object");
    let mut converter = UnitConverterBuilder::new()
        .show_debug_messages(true)
        .include_reversed_conversion(true)
        .add_toml_units("Units.toml")
        .add_toml_conversions("Base_Conversions.toml")
        //.add_conversion("Kilometers", "Meters", 0.01)
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
                    if command == "test" {
                        //graph_test();
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
