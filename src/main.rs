use std::io;

use calculator_rs::{ast::ASTParser, evaluate, scanner::tokenize};

fn main() {
    let mut input = String::new();
    println!("Simple Calculator");
    println!("To calculate, type a formula:");
    loop {
        let _ = io::stdin().read_line(&mut input);
        let tokens = tokenize(input.clone());
        if let Err(e) = tokens {
            println!("{e}");
            continue;
        }
        let ast = ASTParser::create_ast(tokens.unwrap());
        if let Err(e) = ast {
            println!("{e}");
            continue;
        }
        let result = evaluate::evaluate(ast.unwrap());
        println!("{result}");
        input = String::new();
    }
}
