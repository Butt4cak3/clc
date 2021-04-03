#[derive(Debug, Clone)]
pub enum Token {
    Number(String),
    Identifier(String),
    Symbol(String),
    Whitespace(String),
    LeftParenthesis,
    RightParenthesis,
}

impl Token {
    pub fn len(&self) -> usize {
        match self {
            Self::Number(s) => s.len(),
            Self::Identifier(s) => s.len(),
            Self::Symbol(s) => s.len(),
            Self::Whitespace(s) => s.len(),
            Self::LeftParenthesis => 1,
            Self::RightParenthesis => 1,
        }
    }
}

pub struct Tokens<'a> {
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
    "\t\n ".contains(s)
}

fn is_digit(s: char) -> bool {
    "0123456789".contains(s)
}

fn is_letter(s: char) -> bool {
    "abcdefghijklmnopqrstuvwxyz".contains(s)
}

fn is_left_parenthesis(s: char) -> bool {
    s == '('
}

fn is_right_parenthesis(s: char) -> bool {
    s == ')'
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
    } else if is_left_parenthesis(current_char) {
        Ok(Token::LeftParenthesis)
    } else if is_right_parenthesis(current_char) {
        Ok(Token::RightParenthesis)
    } else {
        Ok(Token::Symbol(String::from(&expression[0..1])))
    }
}

pub fn tokenize(expression: &str) -> Tokens {
    Tokens::from(expression)
}
