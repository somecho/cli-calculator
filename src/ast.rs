use crate::token::Token;

#[derive(Debug)]
pub enum Expression {
    Grouping(Box<Expression>),
    Literal(f32),
    Unary(Token, Box<Expression>),
    Binary(Token, Box<Expression>, Box<Expression>),
    SingleArity(Token, Box<Expression>),
    DoubleArity(Token, Box<Expression>, Box<Expression>),
    MultiArity(Token, Vec<Box<Expression>>),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Expression::Grouping(e) => format!("({})", e.to_string()),
            Expression::Literal(n) => n.to_string(),
            Expression::Unary(op, e) => format!("{}{}", op, e.to_string()),
            Expression::Binary(op, l, r) => format!("{} {} {}", l.to_string(), op, r.to_string()),
            Expression::SingleArity(op, a) => format!("{}({})", op, a.to_string()),
            Expression::DoubleArity(op, a, b) => {
                format!("{}({},{})", op, a.to_string(), b.to_string())
            }
            Expression::MultiArity(op, args) => {
                let mut string = String::from(op.to_string());
                string += "(";
                string += &args[0].to_string();
                for i in 1..args.len() {
                    string += ",";
                    string += &args[i].to_string();
                }
                string += ")";
                string
            }
        };
        write!(f, "{}", s)
    }
}

pub struct ASTParser {
    current: usize,
    tokens: Vec<Token>,
}

impl ASTParser {
    pub fn create_ast(tokens: Vec<Token>) -> Result<Box<Expression>, String> {
        let mut parser = ASTParser {
            current: 0,
            tokens: tokens.clone(),
        };
        let expression = parser.expression();
        if parser.current < parser.tokens.len() {
            let disjoint = tokens
                .get(parser.current..parser.tokens.len())
                .unwrap()
                .iter()
                .fold(String::new(), |acc, t| acc + &t.to_string());
            return Err(format!(
                "Error: \"{}\" is disjointed from formula",
                disjoint
            ));
        }
        expression
    }

    fn expression(&mut self) -> Result<Box<Expression>, String> {
        self.term()
    }

    fn function(&mut self) -> Result<Box<Expression>, String> {
        let operator = self.previous().clone();
        if vec![
            Token::Cos,
            Token::Sin,
            Token::Tan,
            Token::Sqrt,
            Token::Floor,
            Token::Ceil,
        ]
        .contains(&operator)
        {
            if self.check(Token::OpenParen) {
                self.advance();
            } else {
                return Err(format!(
                    "Missing opening parenthesis ')' after {}",
                    operator
                ));
            }
            if self.end() {
                return Err("Cannot end with opening parenthesis".to_string());
            }
            let arg = self.expression();
            if self.check(Token::CloseParen) {
                self.advance();
            } else {
                return Err(format!(
                    "Missing closing parenthesis ')' after {}",
                    operator
                ));
            }

            return Ok(Box::new(Expression::SingleArity(
                operator,
                arg.expect("Unable to create syntax tree"),
            )));
        }
        if vec![Token::Pow, Token::Log].contains(&operator) {
            if self.check(Token::OpenParen) {
                self.advance();
            } else {
                return Err(format!(
                    "Missing opening parenthesis '(' after {}",
                    operator
                ));
            }

            if self.end() {
                return Err("Cannot end with opening parenthesis".to_string());
            }

            let arg1 = self.expression();

            if self.check(Token::Comma) {
                self.advance();
            } else {
                return Err(format!("{} requires atleast two operands", operator));
            }

            let arg2 = self.expression();

            if self.check(Token::CloseParen) {
                self.advance();
            } else {
                return Err(format!(
                    "Missing closing parenthesis ')' after {}",
                    operator
                ));
            }
            return Ok(Box::new(Expression::DoubleArity(
                operator,
                arg1.expect("Unable to create syntax tree"),
                arg2.expect("Unable to create syntax tree"),
            )));
        }

        if vec![Token::Max, Token::Min].contains(&operator) {
            if self.check(Token::OpenParen) {
                self.advance();
            } else {
                return Err(format!(
                    "Missing opening parenthesis '(' after {}",
                    operator
                ));
            }
            let mut args: Vec<Box<Expression>> = Vec::new();
            if self.end() {
                return Err("Cannot end with opening parenthesis".to_string());
            }
            while *self.peek() != Token::CloseParen {
                args.push(self.expression().expect("Unable to create syntax tree"));

                if *self.peek() == Token::CloseParen {
                    break;
                }

                if *self.peek() != Token::Comma {
                    return Err("Either missing comma or close parenthesis".to_string());
                }

                self.advance();

                if *self.peek() == Token::CloseParen {
                    return Err("Trailing commas are not allowed".to_string());
                }
            }
            if self.check(Token::CloseParen) {
                self.advance();
            } else {
                return Err(format!(
                    "Missing closing parenthesis ')' after {}",
                    operator
                ));
            }
            if args.len() < 2 {
                return Err(format!("{} requires 2 or more arguments", operator));
            }
            return Ok(Box::new(Expression::MultiArity(operator, args)));
        }
        Err(format!("{} is not a valid function", operator))
    }

    fn term(&mut self) -> Result<Box<Expression>, String> {
        let mut expr = self.factor();
        while self.r#match(&vec![Token::Minus, Token::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Ok(Box::new(Expression::Binary(
                operator,
                expr.expect("unable to create syntax tree"),
                right.expect("unable to create syntax tree"),
            )))
        }
        expr
    }

    fn factor(&mut self) -> Result<Box<Expression>, String> {
        let mut expr = self.unary();
        while self.r#match(&vec![Token::Slash, Token::Star, Token::Percent]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Ok(Box::new(Expression::Binary(
                operator,
                expr.expect("unable to create syntax tree"),
                right.expect("unable to create syntax tree"),
            )));
        }
        expr
    }

    fn unary(&mut self) -> Result<Box<Expression>, String> {
        if self.r#match(&vec![Token::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Ok(Box::new(Expression::Unary(
                operator,
                right.expect("unable to create syntax tree"),
            )));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expression>, String> {
        if self.r#match(&vec![Token::OpenParen]) {
            let expr = self.expression();
            if self.check(Token::CloseParen) {
                self.advance();
            } else {
                return Err("Missing matching parenthesis ')'".to_string());
            }
            return Ok(Box::new(Expression::Grouping(
                expr.expect("Unable to create syntax tree"),
            )));
        }

        if self.r#match(&vec![
            Token::Cos,
            Token::Tan,
            Token::Sin,
            Token::Log,
            Token::Sqrt,
            Token::Max,
            Token::Min,
            Token::Pow,
            Token::Floor,
            Token::Ceil,
        ]) {
            return self.function();
        }

        if self.r#match(&vec![Token::Number(self.peek().get_number()?)]) {
            return Ok(Box::new(Expression::Literal(self.previous().get_number()?)));
        }
        Err("Unable to create syntax tree".to_string())
    }

    /// checks if current token matches given token and consumes it if it does
    fn r#match(&mut self, types: &Vec<Token>) -> bool {
        for t in types.iter() {
            if self.check(*t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: Token) -> bool {
        if self.end() {
            false
        } else {
            *self.peek() == token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current - 1)
            .expect("Previous token not found")
    }

    /// checks for EOF token
    fn end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    /// check current token without consuming it
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).expect("Index out of bounds!")
    }
}
