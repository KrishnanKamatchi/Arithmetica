use std::io;

fn main() {
    let mut placeholder: String = String::new();

    io::stdin()
        .read_line(&mut placeholder)
        .expect("Oops somethign went wrong");

    let input = placeholder.trim();
    let tokens = tokenize(input);
    let expr = parse(&tokens);
    let result = eval(&expr);
    println!("Result: {}", result); // Output should be 21
}

fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinOp(left, op, right) => {
            let left_val = eval(left);
            let right_val = eval(right);
            match op {
                Operator::Add => left_val + right_val,
                Operator::Subtract => left_val - right_val,
                Operator::Multiply => left_val * right_val,
                Operator::Divide => left_val / right_val,
            }
        }
    }
}

#[derive(Debug)]
enum Expr {
    Number(i32),
    BinOp(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn parse(tokens: &[Token]) -> Expr {
    let (expr, _) = parse_expr(tokens);
    expr
}

fn parse_expr(tokens: &[Token]) -> (Expr, &[Token]) {
    let (mut left, mut rest) = parse_term(tokens);

    while let Some(token) = rest.first() {
        match token {
            Token::Plus => {
                let (right, rest_after) = parse_term(&rest[1..]);
                left = Expr::BinOp(Box::new(left), Operator::Add, Box::new(right));
                rest = rest_after;
            }
            Token::Minus => {
                let (right, rest_after) = parse_term(&rest[1..]);
                left = Expr::BinOp(Box::new(left), Operator::Subtract, Box::new(right));
                rest = rest_after;
            }
            _ => break,
        }
    }
    (left, rest)
}

fn parse_term(tokens: &[Token]) -> (Expr, &[Token]) {
    let (mut left, mut rest) = parse_factor(tokens);

    while let Some(token) = rest.first() {
        match token {
            Token::Multiply => {
                let (right, rest_after) = parse_factor(&rest[1..]);
                left = Expr::BinOp(Box::new(left), Operator::Multiply, Box::new(right));
                rest = rest_after;
            }
            Token::Divide => {
                let (right, rest_after) = parse_factor(&rest[1..]);
                left = Expr::BinOp(Box::new(left), Operator::Divide, Box::new(right));
                rest = rest_after;
            }
            _ => break,
        }
    }
    (left, rest)
}

fn parse_factor(tokens: &[Token]) -> (Expr, &[Token]) {
    match tokens.first() {
        Some(Token::Number(n)) => (Expr::Number(*n), &tokens[1..]),
        Some(Token::LParen) => {
            let (expr, rest) = parse_expr(&tokens[1..]);
            if let Some(Token::RParen) = rest.first() {
                return (expr, &rest[1..]);
            }
            panic!("Expected closing parenthesis");
        }
        _ => panic!("Unexpected token: {:?}", tokens),
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    EOF, // End of file/input
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                let mut number = (c.to_digit(10).unwrap()) as i32;
                while let Some(next_char) = chars.peek() {
                    if next_char.is_digit(10) {
                        number = number * 10 + next_char.to_digit(10).unwrap() as i32;
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number));
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            _ if c.is_whitespace() => continue, // Ignore whitespace
            _ => panic!("Unknown character: {}", c),
        }
    }
    tokens.push(Token::EOF); // End of file
    tokens
}
