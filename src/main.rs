#![allow(non_snake_case)]
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if *command == String::from("h") {
        ShowHelp();
    } else if *command == String::from("pow") {
        let base = *&args[2].parse::<i64>().unwrap();
        let exp = *&args[3].parse::<u64>().unwrap();
        println!("{}", Pow(base, exp));
    } else if *command == String::from("sqrt") {
        let num = &args[2].parse::<f64>().unwrap();
        let iters = &args[3].parse::<u32>().unwrap();
        println!("{}", Sqrt(*num, *iters));
    } else if *command == String::from("fac") {
        let num = &args[2].parse::<u64>().unwrap();
        println!("{}", Factorial(*num));
    } else if *command == String::from("fib-upto") {
        let max = &args[2].parse::<u64>().unwrap();
        Fibonacci::UpTo(*max);
    } else if *command == String::from("fib-oflen") {
        let limit = &args[2].parse::<u64>().unwrap();
        Fibonacci::OfLength(*limit);
    } else {
        eprintln!(
            "Error: Invalid command \"{}\". Use command 'h' to see valid commands",
            &command
        );
    }
}

fn ShowHelp() {
    println!("The valid arguments are:");
    println!("h - Show help");
    println!("pow [n] [e] - Calculate [n] to the power of [e]");
    println!("sqrt [n] [i] - Calculate the square root of [n] with [i] iterations");
    println!("fac [n] - Calculate the factorial of [n]");
    println!("fib-upto [n] - Show a fibonacci sequence up to [n]");
    println!("fib-oflen - Show a fibonacci sequence of length n");
}

fn Sqrt(num: f64, iters: u32) -> f64 {
    let mut mean = (num + 1.0) / 2.0;

    for _ in 0..iters {
        let estimate = num / mean;
        mean = (mean + estimate) / 2.0;
    }

    return mean;
}
fn Factorial(n: u64) -> u64 {
    let mut ans = 1;

    for i in 1..=n {
        ans *= i;
    }

    return ans;
}
fn Pow(base: i64, exp: u64) -> i64 {
    let mut ans = 1;

    if exp == 0 {
        return 1;
    }

    for _ in 1..=exp {
        ans *= base;
    }

    return ans;
}

mod Fibonacci {
    pub fn UpTo(n: u64) {
        let (mut a, mut b) = (0, 1);

        while a <= n {
            print!("{} ", a);
            (a, b) = (b, a + b);
        }
    }
    pub fn OfLength(n: u64) {
        let (mut a, mut b) = (0, 1);

        for _ in 0..n {
            print!("{} ", a);
            (a, b) = (b, a + b);
        }
    }
}
