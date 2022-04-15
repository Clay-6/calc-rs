use anyhow::{anyhow, Result};
use chumsky::prelude::*;

pub fn run(src: &str) -> Result<f64> {
    match parser().parse(src) {
        Ok(ast) => match eval(&ast) {
            Ok(ans) => Ok(ans),
            Err(e) => Err(anyhow!("Evaluation error: {}", e)),
        },
        Err(errs) => {
            errs.into_iter()
                .for_each(|e| eprintln!("Parse error: {}", e));
            return Err(anyhow!("Parse Err"));
        }
    }
}

fn eval(expr: &Expr) -> Result<f64, String> {
    use Expr::*;

    match expr {
        Num(n) => Ok(*n),
        Neg(a) => Ok(-eval(a)?),
        Add(a, b) => Ok(eval(a)? + eval(b)?),
        Sub(a, b) => Ok(eval(a)? - eval(b)?),
        Mul(a, b) => Ok(eval(a)? * eval(b)?),
        Div(a, b) => Ok(eval(a)? / eval(b)?),
        Pow(a, b) => Ok(eval(a)?.powf(eval(b)?)),
        Mod(a, b) => Ok(eval(a)? % eval(b)?),
    }
}

enum Expr {
    Num(f64),

    Neg(Box<Self>),

    Pow(Box<Self>, Box<Self>),
    Mod(Box<Self>, Box<Self>),

    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),

    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
}

fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr| {
        let num = text::int(10)
            .map(|s: String| Expr::Num(s.parse().unwrap()))
            .padded();

        let atom = num.or(expr.delimited_by(just('('), just(')')));

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_op, rhs| Expr::Neg(Box::new(rhs)));

        let pow = unary
            .clone()
            .then(
                op('^')
                    .to(Expr::Pow as fn(_, _) -> _)
                    .or(op('%').to(Expr::Mod as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let product = pow
            .clone()
            .then(
                op('*')
                    .to(Expr::Mul as fn(_, _) -> _)
                    .or(op('/').to(Expr::Div as fn(_, _) -> _))
                    .then(pow)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                op('+')
                    .to(Expr::Add as fn(_, _) -> _)
                    .or(op('-').to(Expr::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        sum.padded()
    })
}
