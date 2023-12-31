use self::parser::parse_conversion;

mod graph;
mod parser;
mod units;

pub fn convert(input: &str) -> Option<f32> {
    let conversion = parse_conversion(&input);
    println!("{:?}", conversion.unwrap());
    None
}
