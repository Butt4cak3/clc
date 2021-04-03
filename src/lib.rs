use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

mod parsing;
pub use parsing::{tokenize, Token};

#[derive(Debug, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Operator {
    symbol: String,
    precedence: i32,
    associativity: Associativity,
}

impl Operator {
    pub fn new(symbol: &str, precedence: i32, associativity: Associativity) -> Self {
        Self {
            symbol: String::from(symbol),
            precedence,
            associativity,
        }
    }
}

pub enum Function {
    NativeFunction {
        name: String,
        num_args: usize,
        execute: Box<dyn Fn(&[f64]) -> f64>,
    },
}

impl Function {
    pub fn get_name(&self) -> &str {
        match self {
            Self::NativeFunction { name, .. } => name,
        }
    }

    pub fn apply(&self, stack: &mut Vec<f64>) {
        match self {
            Function::NativeFunction {
                num_args, execute, ..
            } => {
                let mut args = Vec::new();
                for _ in 0..*num_args {
                    if let Some(arg) = stack.pop() {
                        args.push(arg);
                    } else {
                        panic!("Not enough arguments left on stack");
                    }
                }
                args.reverse();
                stack.push(execute(&args));
            }
        }
    }
}

pub struct Context {
    variables: HashMap<String, f64>,
    operators: HashMap<String, Operator>,
    functions: HashMap<String, Function>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            operators: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, value: f64) {
        self.variables.insert(String::from(name), value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&f64> {
        self.variables.get(name)
    }

    pub fn add_operator(&mut self, symbol: &str, precedence: i32, associativity: Associativity) {
        let operator = Operator::new(symbol, precedence, associativity);
        self.operators.insert(String::from(symbol), operator);
    }

    pub fn get_operator(&self, symbol: &str) -> Option<&Operator> {
        self.operators.get(symbol)
    }

    pub fn function_exists(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions
            .insert(function.get_name().to_string(), function);
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }
}

impl Default for Context {
    fn default() -> Self {
        let mut context = Self::new();
        context.add_operator("+", 2, Associativity::Left);
        context.add_operator("-", 2, Associativity::Left);
        context.add_operator("*", 3, Associativity::Left);
        context.add_operator("/", 3, Associativity::Left);
        context.add_operator("^", 4, Associativity::Right);
        context.add_function(Function::NativeFunction {
            name: String::from("min"),
            num_args: 2,
            execute: Box::new(|args| if args[0] < args[1] { args[0] } else { args[1] }),
        });
        context.add_function(Function::NativeFunction {
            name: String::from("max"),
            num_args: 2,
            execute: Box::new(|args| if args[0] > args[1] { args[0] } else { args[1] }),
        });
        context
    }
}

fn move_operators(
    operator: &Operator,
    stack: &mut Vec<Token>,
    queue: &mut VecDeque<Token>,
    context: &Context,
) {
    while let Some(Token::Symbol(other_symbol)) = stack.last() {
        if let Some(other_operator) = context.get_operator(other_symbol) {
            if other_operator.precedence > operator.precedence
                || (other_operator.precedence == operator.precedence
                    && operator.associativity == Associativity::Left)
            {
                if let Some(t) = stack.pop() {
                    queue.push_back(t);
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

pub fn shunting_yard(tokens: Vec<Token>, context: &Context) -> VecDeque<Token> {
    let mut queue: VecDeque<Token> = VecDeque::new();
    let mut stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => queue.push_back(token),
            Token::Identifier(ref name) if context.function_exists(name) => stack.push(token),
            Token::Identifier(_) => queue.push_back(token),
            Token::Symbol(ref symbol) => {
                if let Some(operator) = context.get_operator(&symbol) {
                    move_operators(&operator, &mut stack, &mut queue, context);
                    stack.push(token);
                } else {
                    panic!("Unknown operator {}", symbol);
                }
            }
            Token::Whitespace(_) => (),
            Token::LeftParenthesis => stack.push(token),
            Token::RightParenthesis => {
                while let Some(token) = stack.last() {
                    if let Token::LeftParenthesis = token {
                        stack.pop();
                        if let Some(Token::Identifier(_)) = stack.last() {
                            queue.push_back(stack.pop().unwrap());
                        }
                        break;
                    } else if let Some(token) = stack.pop() {
                        queue.push_back(token);
                    } else {
                        panic!("Mismatched parentheses.");
                    }
                }
            }
            Token::ArgumentSeparator => {
                while let Some(token) = stack.last() {
                    if let Token::LeftParenthesis = token {
                        break;
                    } else {
                        queue.push_back(stack.pop().unwrap());
                    }
                }

                if stack.len() == 0 {
                    panic!("Stack is empty");
                }
            }
        }
    }

    while let Some(token) = stack.pop() {
        queue.push_back(token);
    }

    queue
}

pub fn evaluate_queue(queue: &VecDeque<Token>, context: &Context) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in queue {
        match token {
            Token::Number(num) => stack.push(num.parse().unwrap()),
            Token::Symbol(s) if s == "+" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left + right);
            }
            Token::Symbol(s) if s == "-" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left - right);
            }
            Token::Symbol(s) if s == "*" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left * right);
            }
            Token::Symbol(s) if s == "/" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left / right);
            }
            Token::Symbol(s) if s == "^" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left.powf(right));
            }
            Token::Identifier(name) => {
                if let Some(&value) = context.get_variable(name) {
                    stack.push(value);
                } else if let Some(function) = context.get_function(name) {
                    function.apply(&mut stack);
                } else {
                    panic!("Variable {} does not exist", name);
                }
            }
            _ => (),
        }
    }

    stack.pop().unwrap()
}

pub fn evaluate(expression: &str, context: &Context) -> f64 {
    let tokens: Vec<Token> = tokenize(expression).try_collect().unwrap();

    let queue = shunting_yard(tokens, context);
    evaluate_queue(&queue, context)
}

#[cfg(test)]
mod tests {
    use crate::evaluate;
    use crate::Context;
    use core::f64::consts::PI;

    fn calc(expression: &str) -> f64 {
        let context = Context::default();
        evaluate(expression, &context)
    }

    #[test]
    fn addition() {
        assert_eq!(calc("4 + 9"), 4.0 + 9.0);
    }

    #[test]
    fn subtraction() {
        assert_eq!(calc("25 - 3"), 25.0 - 3.0);
    }

    #[test]
    fn multiplication() {
        assert_eq!(calc("2 * 3"), 2.0 * 3.0);
    }

    #[test]
    fn division() {
        assert_eq!(calc("10 / 2"), 10.0 / 2.0);
    }

    #[test]
    fn exponentiation() {
        assert_eq!(calc("2^10"), f64::powf(2.0, 10.0));
    }

    #[test]
    fn variables() {
        let mut context = Context::default();
        context.set_variable("pi", PI);

        assert_eq!(evaluate("2 * pi", &context), 2.0 * PI);
    }

    #[test]
    fn precedence() {
        assert_eq!(calc("2 + 3 * 4"), 2.0 + 3.0 * 4.0);
    }

    #[test]
    fn parentheses() {
        assert_eq!(calc("(2 + 3) * 4"), (2.0 + 3.0) * 4.0);
        assert_eq!(calc("((2) + 3) * 4"), (2.0 + 3.0) * 4.0);
        assert_eq!(calc("(5 + 3) * (4 - 1)"), (5.0 + 3.0) * (4.0 - 1.0));
        assert_eq!(calc("2^(9+1)"), f64::powf(2.0, 9.0 + 1.0));
    }

    #[test]
    fn functions() {
        assert_eq!(calc("max(2 * 4, 3 + 5)"), 8.0);
    }
}
