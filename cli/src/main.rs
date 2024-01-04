use std::io;

use unitconvert::persistence::in_memory::InMemoryConversionStore;
use unitconvert::units::{LengthUnit, Unit};
use unitconvert::UnitConverter;

fn main() {
    let mut converter = UnitConverter::new();
    let mut store = InMemoryConversionStore::new();
    store.insert(
        Unit::Length(LengthUnit::Kilometers),
        Unit::Length(LengthUnit::Meters),
        0.001,
    );

    converter.add_default_conversions(&store);

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
