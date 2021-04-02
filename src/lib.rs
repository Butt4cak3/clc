use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

mod parsing;
pub use parsing::{tokenize, Token};

#[derive(Debug)]
pub struct Context {
    variables: HashMap<String, f64>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, value: f64) {
        self.variables.insert(String::from(name), value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&f64> {
        self.variables.get(name)
    }
}

pub fn shunting_yard(tokens: Vec<Token>) -> VecDeque<Token> {
    let mut queue: VecDeque<Token> = VecDeque::new();
    let mut stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => queue.push_back(token),
            Token::Identifier(_) => queue.push_back(token),
            Token::Symbol(_) => {
                while stack.len() > 0 {
                    queue.push_back(stack.pop().unwrap());
                }

                stack.push(token);
            }
            Token::Whitespace(_) => (),
        }
    }

    for token in stack.into_iter().rev() {
        queue.push_back(token);
    }

    queue
}

pub fn evaluate_queue(queue: &VecDeque<Token>, context: &Context) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in queue {
        match token {
            Token::Number(num) => stack.push(num.parse().unwrap()),
            Token::Symbol(s) if s == "*" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left * right);
            }
            Token::Identifier(name) => {
                let variable = context.get_variable(name);
                if let Some(&value) = variable {
                    stack.push(value);
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

    let queue = shunting_yard(tokens);
    evaluate_queue(&queue, &context)
}

#[cfg(test)]
mod tests {
    use crate::evaluate;
    use crate::Context;

    fn calc(expression: &str) -> f64 {
        let context = Context::new();
        evaluate(expression, &context)
    }

    #[test]
    fn addition() {
        assert_eq!(calc("4 + 9"), 15.0);
    }

    #[test]
    fn subtraction() {
        assert_eq!(calc("25 - 3"), 22.0);
    }

    #[test]
    fn multiplication() {
        assert_eq!(calc("2 * 3"), 6.0);
    }

    #[test]
    fn division() {
        assert_eq!(calc("10 / 2"), 5.0);
    }
}
