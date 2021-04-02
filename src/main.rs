use core::f64::consts::PI;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Context {
    variables: HashMap<String, f64>,
}

impl Context {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    fn set_variable(&mut self, name: &str, value: f64) {
        self.variables.insert(String::from(name), value);
    }

    fn get_variable(&self, name: &str) -> Option<&f64> {
        self.variables.get(name)
    }
}

#[derive(Debug, Clone)]
enum Token {
    Number(String),
    Identifier(String),
    Symbol(String),
    Whitespace(String),
}

impl Token {
    fn len(&self) -> usize {
        match self {
            Self::Number(s) => s.len(),
            Self::Identifier(s) => s.len(),
            Self::Symbol(s) => s.len(),
            Self::Whitespace(s) => s.len(),
        }
    }
}

struct Tokens<'a> {
    expression: &'a str,
    pos: usize,
    error: bool,
}

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(expression: &'a str) -> Self {
        Self {
            expression,
            pos: 0,
            error: false,
        }
    }
}

impl Iterator for Tokens<'_> {
    type Item = Result<Token, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error || self.pos >= self.expression.len() {
            return None;
        }

        let res = parse_token(&self.expression[self.pos..]);
        if let Ok(token) = &res {
            self.pos += token.len();
        } else {
            self.error = true;
        }
        Some(res)
    }
}

fn is_whitespace(s: char) -> bool {
    s == ' '
}

fn is_digit(s: char) -> bool {
    "0123456789".contains(s)
}

fn is_letter(s: char) -> bool {
    "abcdefghijklmnopqrstuvwxyz".contains(s)
}

fn parse_whitespace(expression: &str) -> &str {
    let mut length = 0;
    for c in expression.chars() {
        if !is_whitespace(c) {
            break;
        }

        length += 1;
    }
    &expression[0..length]
}

fn parse_number(expression: &str) -> Result<&str, &'static str> {
    let mut length = 0;
    let mut has_decimals = false;
    let mut last_char: Option<char> = None;

    for c in expression.chars() {
        if c == '.' && has_decimals {
            return Err("Malformed number");
        } else if c == '.' && !has_decimals {
            has_decimals = true;
        } else if !is_digit(c) {
            break;
        }

        last_char = Some(c);
        length += 1;
    }

    if let Some('.') = last_char {
        return Err("Maldformed number");
    }

    Ok(&expression[0..length])
}

fn parse_identifier(expression: &str) -> &str {
    let mut length = 0;
    for c in expression.chars() {
        if !is_letter(c) {
            break;
        }

        length += 1;
    }
    &expression[0..length]
}

fn parse_token(expression: &str) -> Result<Token, &'static str> {
    let current_char = expression.chars().nth(0).unwrap();

    if is_whitespace(current_char) {
        Ok(Token::Whitespace(parse_whitespace(&expression).to_owned()))
    } else if is_digit(current_char) {
        Ok(Token::Number(parse_number(&expression)?.to_owned()))
    } else if is_letter(current_char) {
        Ok(Token::Identifier(parse_identifier(&expression).to_owned()))
    } else {
        Ok(Token::Symbol(String::from(&expression[0..1])))
    }
}

fn tokenize(expression: &str) -> Tokens {
    Tokens::from(expression)
}

fn shunting_yard(tokens: Vec<Token>) -> VecDeque<Token> {
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

fn evaluate(queue: &VecDeque<Token>, context: &Context) -> f64 {
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

fn main() {
    let mut context = Context::new();
    context.set_variable("pi", PI);

    let expression = "2 * pi";

    let tokens: Vec<Token> = tokenize(expression).try_collect().unwrap();

    let queue = shunting_yard(tokens);
    let result = evaluate(&queue, &context);

    println!("{} = {}", expression, result);
}
