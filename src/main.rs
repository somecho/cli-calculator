use calculator_rs::{ast::ASTParser, scanner::tokenize};

fn main() {
    let input = String::from("4 * (1 + 2)");
    println!("{}", input);
    let tokens = tokenize(input);
    for token in tokens.clone().unwrap().iter() {
        println!("{}", token);
    }

    let ast = ASTParser::create_ast(tokens.clone().unwrap());

    println!("{:?}",ast.unwrap());
}
