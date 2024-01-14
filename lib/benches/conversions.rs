use core::time;
use std::thread;

use criterion::{criterion_group, criterion_main, Criterion};
use unitconvert::converter::builder::UnitConverterBuilder;
use unitconvert::converter::UnitConverter;

fn setup() -> UnitConverter {
    let unit_definitions_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Units.toml");
    let default_converions_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Base_Conversions.toml");
    UnitConverterBuilder::new()
        .auto_reverse_conversions(true)
        .add_unit_definitions_toml(unit_definitions_path)
        .add_default_conversions_toml(default_converions_path)
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
