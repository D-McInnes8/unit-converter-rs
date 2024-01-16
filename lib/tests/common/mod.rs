use unitconvert::converter::builder::UnitConverterBuilder;
use unitconvert::converter::UnitConverter;

pub fn setup() -> UnitConverter {
    let unit_definitions_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Units.toml");
    let default_converions_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Base_Conversions.toml");
    UnitConverterBuilder::new()
        .reverse_base_conversions(true)
        .cache_results(true)
        .add_unit_definitions_toml(unit_definitions_path)
        .add_default_conversions_toml(default_converions_path)
        .build()
        .unwrap()
}
