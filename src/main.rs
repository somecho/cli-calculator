use calculator_rs::scanner::tokenize;

fn main() {
    let input = String::from("4 * (8 + 2) * min(max(3.14,2.14), 5*24)");
    println!("{}", input);
    let tokens = tokenize(input);
    for token in tokens.unwrap().iter() {
        println!("{}", token);
    }
}
