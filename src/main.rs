#![allow(non_snake_case)]
use std::env;

use Calculator::{Arithmetic, Fibonacci, Utils};

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if *command == String::from("h") {
        Utils::ShowHelp();
    } else if *command == String::from("pow") {
        let base = *&args[2].parse::<f64>().expect("Please enter a valid number");
        let exp = *&args[3]
            .parse::<i64>()
            .expect("Please enter a positive whole number");
        println!("{}", Arithmetic::Pow(base, exp));
    } else if *command == String::from("sqrt") {
        let num = &args[2].parse::<f64>().expect("Please enter a valid number");
        let iters = &args[3]
            .parse::<u32>()
            .expect("Please enter a positive whole number");
        println!("{}", Arithmetic::Sqrt(*num, *iters));
    } else if *command == String::from("fac") {
        let num = &args[2]
            .parse::<u64>()
            .expect("Please enter a positive whole number");
        println!("{}", Arithmetic::Factorial(*num));
    } else if *command == String::from("fib-upto") {
        let max = &args[2]
            .parse::<u64>()
            .expect("Please enter a positive whole number");
        Fibonacci::UpTo(*max);
    } else if *command == String::from("fib-oflen") {
        let limit = &args[2]
            .parse::<u64>()
            .expect("Please enter a positive whole number");
        Fibonacci::OfLength(*limit);
    } else {
        eprintln!(
            "Error: Invalid command \"{}\". Use command 'h' to see valid commands",
            &command
        );
    }
}
