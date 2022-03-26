#![allow(non_snake_case)]
use clap::Parser;

use Calculator::{Arithmetic, Fibonacci};

fn main() {
    let args = Args::parse();

    match args.operation.as_str() {
        "add" | "+" => {
            println!("{}", Arithmetic::Add(args.a, args.b));
        }
        "sub" | "-" => {
            println!("{}", Arithmetic::Subtract(args.a, args.b));
        }
        "mult" | "*" => {
            println!("{}", Arithmetic::Multiply(args.a, args.b));
        }
        "div" | "/" => {
            println!("{}", Arithmetic::Divide(args.a, args.b));
        }
        "quot" => {
            println!("{}", Arithmetic::Quotient(args.a, args.b));
        }
        "mod" | "%" => {
            println!("{}", Arithmetic::Modulo(args.a, args.b));
        }
        "pow" | "^" => {
            println!("{}", Arithmetic::Pow(args.a, Some(args.b as i64)));
        }
        "sqrt" => {
            println!("{}", Arithmetic::Sqrt(args.a, Some(args.b as u32)));
        }
        "fac" | "!" => {
            println!("{}", Arithmetic::Factorial(args.a as i64));
        }
        "fib-upto" => {
            Fibonacci::UpTo(args.a as u64);
        }
        "fib-oflen" => {
            Fibonacci::OfLength(args.a as u64);
        }
        cmd => eprintln!("Invalid command \"{}\" Use -h or --help for help", cmd),
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The operation to perform.
    /// Possible values: add, sub, mult, div, quot, mod, pow, sqrt, fac, fib-upto, fib-oflen.
    /// Can be substituted their mathematical symbol equivalents
    /// (e.g. +, -, *, /, %, ^, sqrt, !, fib-upto, fib-oflen)
    #[clap(short, long)]
    operation: String,
    a: f64,
    #[clap(default_value_t = 0.0)]
    b: f64,
}
