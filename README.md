# Unit Converter

A tool written in Rust that can be used to convert units from one type to another.

# Crates

## expr

A crate for evaluating expressions, used by the unit-converter crate when conversions require complex expressions rather than simple multiplications, i.e. when converting between temperatures.

An expression can be evaluated using two different methods, the first is the `eval` function, which takes an expression as a string as returns the result.

```
let result = eval(5 + 10);
assert_eq!(result, 15.0);```

The second method is using the `Expression` struct, which compiles the expression into an abstract syntax tree which can be evaluated multiple times. An expression can be evaluated without parameters by calling the `eval` function:

```
if let Ok(expr) = Expression::new("5 + 10") {
  let result = expr.eval();
  assert_eq!(result, 15.0);
}
```

Otherwise the `eval_with_ctx` function can be used, which takes an `ExpressionContext`. Expression contexts can be used to pass parameters into an expression.

```
if let Ok(expr) = Expression:new("a + 10") {
  let mut ctx = InMemoryExpressionContext::default();
  ctx.var("a", 5);

  let result = expr.eval_with_ctx(&ctx);
  assert_eq!(result, 15.0);
}
```

## unit-converter

The main package used to convert units, this package uses a graph algorithm to convert a unit of one type to any other unit of that same type. Conversions can be passed in as text and the result will be returned.

```
let converter = UnitConverterBuilder::new()
  .add_unit_definitions(units)
  .add_base_conversions(conversions)
  .buld();

let result = converter.convert_from_expression("1km -> m");
assert_eq!(result, 1000.0);
```
