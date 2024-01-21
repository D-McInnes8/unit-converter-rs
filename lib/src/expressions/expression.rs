#[derive(Debug, PartialEq)]
pub enum OperationType {
    Number { value: f64 },
    BinaryExpression { operator: Operator },
    Function { name: String, value: f64 },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq)]
pub enum Function {
    Sin,
}
