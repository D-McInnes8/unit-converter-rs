use unitconvert::UnitConverterBuilder;

#[test]
fn convert_kilometers_to_meters() {
    let mut converter = UnitConverterBuilder::new()
        .include_reversed_conversion(true)
        .add_conversion("Meters", "Kilometers", 0.001)
        .build();

    let actual = converter.convert_from_expression("1k -> m");
    assert_eq!(actual.ok(), Some(1000.0));
}
