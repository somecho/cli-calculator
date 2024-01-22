use std::collections::HashMap;

use crate::{ast::Expression, token::Token};

pub fn evaluate(expr: Box<Expression>, variables: &mut HashMap<String, f32>) -> f32 {
    match *expr {
        Expression::Grouping(e) => evaluate(e, variables),
        Expression::Literal(n) => n,
        Expression::Unary(op, v) => {
            let value = evaluate(v, variables);
            if op == Token::Minus {
                return -value;
            }
            unreachable!()
        }
        Expression::Binary(op, a, b) => {
            let a = evaluate(a, variables);
            let b = evaluate(b, variables);
            match op {
                Token::Minus => a - b,
                Token::Plus => a + b,
                Token::Star => a * b,
                Token::Slash => a / b,
                Token::Percent => a % b,
                _ => unreachable!(),
            }
        }
        Expression::SingleArity(op, a) => {
            let a = evaluate(a, variables);
            match op {
                Token::Sqrt => a.sqrt(),
                Token::Floor => a.floor(),
                Token::Ceil => a.ceil(),
                Token::Cos => a.cos(),
                Token::Sin => a.sin(),
                Token::Tan => a.tan(),
                _ => unreachable!(),
            }
        }
        Expression::DoubleArity(op, a, b) => {
            let a = evaluate(a, variables);
            let b = evaluate(b, variables);
            match op {
                Token::Pow => a.powf(b),
                Token::Log => a.log(b),
                _ => unreachable!(),
            }
        }
        Expression::MultiArity(op, args) => {
            let args: Vec<f32> = args
                .into_iter()
                .map(|arg| evaluate(arg, variables))
                .collect();
            match op {
                Token::Max => args
                    .into_iter()
                    .max_by(|&a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
                Token::Min => args
                    .into_iter()
                    .min_by(|&a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
                _ => unreachable!(),
            }
        }
        Expression::Assignment(t, e) => {
            let v = evaluate(e, variables);
            variables.insert(t.get_identifier().unwrap(), v);
            Default::default()
        }
        Expression::Variable(s) => *variables.get(&s).unwrap(),
    }
}
