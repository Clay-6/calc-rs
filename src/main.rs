#![allow(non_snake_case)]
use std::env;

use Calculator::{Arithmetic, Fibonacci, Utils};

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from(" ")
    };

    if command == String::from("h") {
        Utils::ShowHelp();
    }

    match command.as_str() {
        "add" | "+" => {
            let x: f64 = *&args[2].parse().expect("Please enter a valid number");
            let y: f64 = *&args[3].parse().expect("Please enter a valid number");
            println!("{}", Arithmetic::Add(x, y));
        }
        "sub" | "-" => {
            let x: f64 = *&args[2].parse().expect("Please enter a valid number");
            let y: f64 = *&args[3].parse().expect("Please enter a valid number");
            println!("{}", Arithmetic::Subtract(x, y));
        }
        "mult" | "*" => {
            let x: f64 = *&args[2].parse().expect("Please enter a valid number");
            let y: f64 = *&args[3].parse().expect("Please enter a valid number");
            println!("{}", Arithmetic::Multiply(x, y));
        }
        "div" | "/" => {
            let x: f64 = *&args[2].parse().expect("Please enter a valid number");
            let y: f64 = *&args[3].parse().expect("Please enter a valid number");
            println!("{}", Arithmetic::Divide(x, y));
        }
        "quot" => {
            let x: f64 = *&args[2].parse().expect("Please enter a valid number");
            let y: f64 = *&args[3].parse().expect("Please enter a valid number");
            println!("{}", Arithmetic::Quotient(x, y));
        }
        "mod" | "%" => {
            let x: f64 = *&args[2].parse().expect("Please enter a vaid number");
            let y: f64 = *&args[3].parse().expect("Please enter a valid number");
            println!("{}", Arithmetic::Mod(x, y));
        }
        "pow" | "^" => {
            let base: f64 = *&args[2].parse().expect("Please enter a valid number");
            let exp = if args.len() > 3 {
                Some(*&args[3].parse::<i64>().expect("Please enter a whole number"))
            } else {
                None
            };
            println!("{}", Arithmetic::Pow(base, exp));
        }
        "sqrt" => {
            let num: f64 = *&args[2].parse().expect("Please enter a valid number");
            let iters: Option<u32> = if args.len() > 3 {
                Some(*&args[3].parse().expect("Please enter a whole number"))
            } else {
                None
            };
            println!("{}", Arithmetic::Sqrt(num, iters));
        }
        "fac" | "!" => {
            let num: i64 = *&args[2]
                .parse()
                .expect("Please enter a positive whole number");
            println!("{}", Arithmetic::Factorial(num));
        }
        "fib-upto" => {
            let limit: u64 = *&args[2]
                .parse()
                .expect("Please enter a positive whole number");
            Fibonacci::UpTo(limit);
        }
        "fib-oflen" => {
            let length: u64 = *&args[2].parse().expect("Please enter a valid number");
            Fibonacci::OfLength(length);
        }
        cmd => eprintln!(
            "Invalid command \"{}\" Use command 'h' to see valid commands",
            cmd
        ),
    }
}
