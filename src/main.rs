mod cli;

use calcrs::{arithmetic, evaluation};
use clap::Parser as _;
use cli::{Args, Operation};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.operation {
        Operation::Add { operands } => println!("{}", operands.iter().sum::<f64>()),
        Operation::Sub { operands } => println!("{}", arithmetic::sub(operands)),
        Operation::Mul { operands } => println!("{}", operands.iter().product::<f64>()),
        Operation::Div { operands } => println!("{}", arithmetic::div(operands)),
        Operation::Mod { a, b } => println!("{}", a % b),
        Operation::Quot { a, b } => println!("{}", (a / b).floor()),
        Operation::Pow { base, exponent } => println!("{}", arithmetic::pow(base, exponent)),
        Operation::Sqrt { n, iters } => println!("{}", arithmetic::sqrt(n, iters)),
        Operation::Fact { n } => println!("{}", arithmetic::factorial(n)),
        Operation::Quadratic { a, b, c } => {
            let (x1, x2) = arithmetic::quadratic(a, b, c)?;
            println!("x1: {x1}, x2: {x2}");
        }
        Operation::Sin { x } => println!("{}", x.sin()),
        Operation::Cos { x } => println!("{}", x.cos()),
        Operation::Tan { x } => println!("{}", x.tan()),
        Operation::Eval { equation } => println!("{}", evaluation::run(&equation)?),
    }

    Ok(())
}
