# Cli Calculator

This is a simple cli calculator created in Rust. It's a personal project to learn about interpreters. The calculator is a scientific calculator with a limited feature set:
- arithmetic operations: `-`, `+`, `*`, `/` and `%`  
- trigonometric functions: `cos`, `sin` and `tan`
- other functions: `max`, `min`, `floor`, `ceil`, `log`

Additionally, this calculator allows the definitions of variables with the `let` keyword.

## Usage
After running the following commands, you will enter a REPL where you can type in mathematical expressions to be calculated.
```
git clone https://github.com/somecho/cli-calculator
cd cli-calculator
cargo run
```

### Syntax
This calculator uses conventional mathematical notation (i.e. infix). For functions, the syntax reads a little bit more like programming languages. The syntax for functions is `FUNCTION(ARG)` for functions with single arity, `FUNCTION(ARG,ARG)` for functions with double aritoes and `FUNCTION(ARG,ARG,...)` for functions with multiple arities. 

#### Examples
- Single arity: `cos(3.1415)`
- Double arity: `pow(2,5.14)`
- Multiple arity:  `min(1,2,3,4,7)`

Expressions can be nested. `min(cos(max(5,10)),pow(10,2),log(100))` would be a valid expression. 

### Variables
Variables can be defined using the `let` keyword.
```
let a = 10
a * 10
=> returns 100
```
## Architecture
The program follows a simple architecture, first [scanning](./src/scanner.rs) the input string to produce a list of tokens. The tokens then get parsed using recursive descent into an [abstract syntax tree](./src/ast.rs), which then finally can be [evaluated](./src/evaluate.rs).
  
