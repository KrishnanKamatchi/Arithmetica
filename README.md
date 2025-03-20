# Arithmetica
A basic rust compiler that can compile math (Rust learning series)

# Description

This project implements a basic arithmetic compiler in Rust. It can evaluate mathematical expressions consisting of addition, subtraction, multiplication, and division. The program reads an expression from the user, tokenizes it, parses it into an Abstract Syntax Tree (AST), and evaluates the result.

## Features

- **Tokenization**: Converts the input expression into tokens (numbers, operators, parentheses).
- **Parsing**: Parses the tokens into an Abstract Syntax Tree (AST).
- **Evaluation**: Evaluates the AST to compute the result.

## Prerequisites

- Rust must be installed. You can install it via [rustup](https://rust-lang.org/).

## How to Run

1. Clone the repository to your local machine:
   ```bash
   git clone https://github.com/KrishnanKamatchi/Arithmetica.git
   cd Arithmetica
   cargo run || cargo build
