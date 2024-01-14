use criterion::{criterion_group, criterion_main, Criterion};
use unitconvert::converter::builder::UnitConverterBuilder;

fn builder_benchmark(c: &mut Criterion) {
    let unit_definitions_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Units.toml");
    let default_converions_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Base_Conversions.toml");

    c.bench_function("builder", |b| {
        b.iter(|| {
            _ = UnitConverterBuilder::new()
                .auto_reverse_conversions(true)
                .add_unit_definitions_toml(unit_definitions_path)
                .add_default_conversions_toml(default_converions_path)
                .build();
        })
    });
}

criterion_group!(benches, builder_benchmark);
criterion_main!(benches);
