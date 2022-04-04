#![allow(non_snake_case)]

use clap::Parser;
use Calculator::{Arithmetic, Fibonacci};

static PI: f64 = std::f64::consts::PI;
static EULER: f64 = std::f64::consts::E;

fn main() {
    let args = Args::parse();

    let a: f64 = match args.a.to_lowercase().as_str() {
        "pi" => PI,
        "e" => EULER,
        num => num.parse().unwrap(),
    };
    let b: f64 = match args.b.to_lowercase().as_str() {
        "pi" => PI,
        "e" => EULER,
        num => num.parse().unwrap(),
    };

    match args.operation.as_str() {
        "add" | "+" => println!("{}", Arithmetic::Add(a, b)),

        "sub" | "-" => println!("{}", Arithmetic::Subtract(a, b)),

        "mult" | "*" => println!("{}", Arithmetic::Multiply(a, b)),

        "div" | "/" => println!("{}", Arithmetic::Divide(a, b)),

        "quot" => println!("{}", Arithmetic::Quotient(a, b)),

        "mod" | "%" => println!("{}", Arithmetic::Modulo(a, b)),

        "pow" | "^" => println!("{}", Arithmetic::Pow(a, Some(b as i64))),

        "sqrt" => println!("{}", Arithmetic::Sqrt(a, b as u32)),

        "fac" | "!" => println!("{}", Arithmetic::Factorial(a as i64)),

        "quadratic" => {
            let c = match args.c.to_lowercase().as_str() {
                "pi" => PI,
                "e" => EULER,
                num => num.parse().unwrap(),
            };
            let ans = Arithmetic::QuadraticFormula(a, b, c);
            if ans == (f64::NAN, f64::NAN) {
                println!("No real number solutions exist");
            } else {
                println!("x1 = {}, x2 = {}", ans.0, ans.1);
            }
        }

        "fib-upto" => Fibonacci::UpTo(a as u64),

        "fib-oflen" => Fibonacci::OfLength(a as u64),

        cmd => eprintln!("Invalid command \"{}\" Use -h or --help for help", cmd),
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The operation to perform
    ///
    /// Possible values (Symbol aliases shown in square brackets if available):
    /// add[+], sub[-], mult[*], div[/], quot, mod[%], pow[^], sqrt, fac[!], quadratic, fib-upto, fib-oflen.
    #[clap(short, long)]
    operation: String,
    /// The first number to use in the operation
    a: String,
    /// The second number to use in the operation
    #[clap(default_value = "0.0")]
    b: String,
    /// The third number to use in the operation. Only used for quadratic formula
    #[clap(default_value = "0.0")]
    c: String,
}
