use criterion::{criterion_group, criterion_main, Criterion};
use unitconvert::converter::builder::UnitConverterBuilder;
use unitconvert::source::toml::conversions::BaseConversionsSourceToml;
use unitconvert::source::toml::units::UnitDefinitionSourceToml;

fn builder_benchmark(c: &mut Criterion) {
    let unit_definitions_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Units.toml");
    let default_converions_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Base_Conversions.toml");

    c.bench_function("builder", |b| {
        b.iter(|| {
            let conversions = BaseConversionsSourceToml::new(default_converions_path)
                .load()
                .unwrap();
            let units = UnitDefinitionSourceToml::new(unit_definitions_path)
                .load()
                .unwrap();
            _ = UnitConverterBuilder::new()
                .reverse_base_conversions(true)
                .cache_results(true)
                .add_unit_definitions(units)
                .add_base_conversions(conversions)
                .build();
        })
    });
}

criterion_group!(benches, builder_benchmark);
criterion_main!(benches);
