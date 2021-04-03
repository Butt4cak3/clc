use clc::{evaluate, Context};
use core::f64::consts::PI;
use std::io::{self, Write};

fn main() {
    let mut context = Context::default();
    context.set_variable("pi", PI);

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().expect("Input error");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input == "" {
            println!("");
            break;
        } else if input.trim() == "" {
            continue;
        }

        let result = evaluate(input.trim(), &context);
        println!("{}\n", result);
    }
}
