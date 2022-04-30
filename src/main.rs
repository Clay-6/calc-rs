mod cli;

use calculator::{arithmetic, evaluation};
use clap::Parser;
use cli::{Args, Operation};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.operation {
        Operation::Add { a, b } => println!("{}", a + b),
        Operation::Sub { a, b } => println!("{}", a - b),
        Operation::Mul { a, b } => println!("{}", a * b),
        Operation::Div { a, b } => println!("{}", a / b),
        Operation::Mod { a, b } => println!("{}", a % b),
        Operation::Quot { a, b } => println!("{}", (a / b).floor()),
        Operation::Pow {
            base: a,
            exponent: b,
        } => println!("{}", arithmetic::pow(a, b)),
        Operation::Sqrt { n, iters } => println!("{}", arithmetic::sqrt(n, iters)),
        Operation::Fact { n } => println!("{}", arithmetic::factorial(n)),
        Operation::Quadratic { a, b, c } => {
            let (x1, x2) = arithmetic::quadratic_formula(a, b, c)?;
            println!("x1: {x1}, x2: {x2}");
        }
        Operation::Eval { equation } => println!("{}", evaluation::run(&equation)?),
    }

    Ok(())
}
