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

// precedence
// grouping / primary
// - (unary)
// * / (factor)
// + - (term)
// Functions

pub struct ASTParser {
    current: usize,
    tokens: Vec<Token>,
}

impl ASTParser {
    pub fn create_ast(tokens: Vec<Token>) -> Result<Box<Expression>, String> {
        let mut parser = ASTParser { current: 0, tokens };
        let expression = parser.expression();
        expression
    }

    fn expression(&mut self) -> Result<Box<Expression>, String> {
        self.term()
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
        while self.r#match(&vec![Token::Slash, Token::Star]) {
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

        if self.r#match(&vec![Token::Number(self.peek().get_number()?)]) {
            return Ok(Box::new(Expression::Literal(self.previous().get_number()?)));
        }
        Err("Unable to create syntax tree".to_string())
    }

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
