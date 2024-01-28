use simple_logger::SimpleLogger;
use unitconvert::converter::builder::UnitConverterBuilder;
use unitconvert::converter::UnitConverter;
use unitconvert::source::toml::conversions::BaseConversionsSourceToml;
use unitconvert::source::toml::units::UnitDefinitionSourceToml;

pub fn setup() -> UnitConverter {
    let unit_definitions_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Units.toml");
    let default_converions_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Base_Conversions.toml");

    let conversions = BaseConversionsSourceToml::new(default_converions_path)
        .load()
        .unwrap();
    let units = UnitDefinitionSourceToml::new(unit_definitions_path)
        .load()
        .unwrap();

    UnitConverterBuilder::new()
        .reverse_base_conversions(true)
        .cache_results(true)
        .add_unit_definitions(units)
        .add_base_conversions(conversions)
        .build()
        .unwrap()
}

pub fn setup_test_logger() {
    let result = SimpleLogger::new().init();
    if result.is_err() {
        eprintln!("Error initialising logger");
    }
}
