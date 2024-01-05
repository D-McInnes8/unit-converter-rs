use unitconvert::converter::builder::UnitConverterBuilder;

#[test]
fn convert_kilometers_to_meters() {
    let mut converter = UnitConverterBuilder::new()
        .include_reversed_conversion(true)
        .add_unit_definition("Length", "Meters", "m")
        .add_unit_definition("Length", "Kilometers", "k")
        .add_conversion("Length", "Meters", "Kilometers", 0.001)
        .build();

    let actual = converter.convert_from_expression("1k -> m");
    assert!(actual.is_ok(), "Returned error {:?}", actual.err());
    assert_eq!(actual.unwrap(), 1000.0);
}
