use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Operator { //operator types
    Add,
    Sub,
    Mul,
    Div,
    Exp
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Token { //token types
    Number(u32),
    Op(Operator),
    Bracket(char)
}
#[derive(Debug)] //error types
pub enum Error {
    BadeToken(char),
    MismatchedParens,
    Equation
}

pub struct Calculator {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BadeToken(c) => write!(f, "Bad token: {}", c),
            Error::MismatchedParens => write!(f, "Mismatched parentheses"),
            Error::Equation => write!(f, "Did you mean to graph?")
        }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::Equation, Error::Equation) => true,
            _ => false,
        }
    }
}

impl std::error::Error for Error {}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens = Vec::new();
        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        *n = *n * 10 + (c as u32 - 48);
                    },
                    _ => {
                        let digit = c as u32 - 48;
                        tokens.push(Token::Number(digit));
                    }
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(c)
                }
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p!= '(' {
                            return Err(Error::MismatchedParens);
                        }
                    } else {
                        return Err(Error::MismatchedParens);
                    }
                },
                '+' => tokens.push(Token::Op(Operator::Add)),
                '-' => tokens.push(Token::Op(Operator::Sub)),
                '*' => tokens.push(Token::Op(Operator::Mul)),
                '/' => tokens.push(Token::Op(Operator::Div)),
                '^' => tokens.push(Token::Op(Operator::Exp)),
                ' ' => {},
                '\n' => {},
                '=' => return Err(Error::Equation),
                'f' => return Err(Error::Equation),
                'y' => return Err(Error::Equation),
                _ => return Err(Error::BadeToken(c))
            }
        }
        if parens.len() > 0 {
            return Err(Error::MismatchedParens);
        }

        Ok(tokens)
    }
    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                Token::Op(_) => {
                    while !stack.is_empty() && stack[stack.len() - 1] >= token {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                },
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('(') {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                },
                _ => {}
            }
        }
        while stack.len() > 0 {
            queue.push(stack.pop().unwrap());
        }
        queue
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
        tokens.reverse();

        let mut stack: Vec<f32> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => stack.push(num as f32),
                Token::Op(Operator::Add) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left + right);
                },
                Token::Op(Operator::Sub) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left - right);
                },
                Token::Op(Operator::Mul) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left * right);
                },
                Token::Op(Operator::Div) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left / right);
                },
                Token::Op(Operator::Exp) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left.powf(right));
                },
                _ => {}
            }
        }
        if stack.len() > 1 {
            None
        } else {
            stack.pop()
        }
    }
   
}