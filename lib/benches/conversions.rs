use core::time;
use std::thread;

use criterion::{criterion_group, criterion_main, Criterion};
use unitconvert::converter::builder::UnitConverterBuilder;
use unitconvert::converter::UnitConverter;
use unitconvert::source::toml::conversions::BaseConversionsSourceToml;
use unitconvert::source::toml::units::UnitDefinitionSourceToml;

fn setup() -> UnitConverter {
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
        .cache_results(false)
        .add_unit_definitions(units)
        .add_base_conversions(conversions)
        .build()
        .unwrap()
}

fn convert_kilometers_to_nautical_miles(c: &mut Criterion) {
    let mut converter = setup();

    thread::sleep(time::Duration::from_secs(5));

    c.bench_function("kilometers to nautical miles", |b| {
        b.iter(|| _ = converter.convert_from_expression("2km -> nmi"))
    });
}

fn convert_meters_to_kilometers(c: &mut Criterion) {
    let mut converter = setup();

    c.bench_function("meters to kilometers", |b| {
        b.iter(|| _ = converter.convert_from_expression("3409km -> m"))
    });
}

fn convert_lightyears_to_nanometers(c: &mut Criterion) {
    let mut converter = setup();

    c.bench_function("lightyears to nanometers", |b| {
        b.iter(|| _ = converter.convert_from_expression("3ly -> nm"))
    });
}

criterion_group!(
    benches,
    convert_kilometers_to_nautical_miles,
    convert_meters_to_kilometers,
    convert_lightyears_to_nanometers
);
criterion_main!(benches);
