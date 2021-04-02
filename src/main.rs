use clc::{evaluate, Context};
use core::f64::consts::PI;

fn main() {
    let mut context = Context::new();
    context.set_variable("pi", PI);

    let expression = "2 * pi";
    let result = evaluate(expression, &context);

    println!("{} = {}", expression, result);
}
