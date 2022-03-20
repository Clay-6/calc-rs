#![allow(non_snake_case)]

pub mod Utils {
    pub fn ShowHelp() {
        println!("The valid commands are:");
        println!("h - Show help");
        println!("pow [n] [e] - Calculate [n] to the power of [e]");
        println!(
            "sqrt [n] [i] - Calculate the square root of [n] with [i] iterations (defaults to 10)"
        );
        println!("fac [n] - Calculate the factorial of [n]");
        println!("fib-upto [n] - Show a fibonacci sequence up to [n]");
        println!("fib-oflen [n] - Show a fibonacci sequence of length [n]");
    }
}
pub mod Arithmetic {
    pub fn Sqrt(num: f64, iters: Option<u32>) -> f64 {
        let mut mean = (num + 1.0) / 2.0;
        let iters = match iters {
            Some(n) => n,
            None => 10,
        };

        for _ in 0..iters {
            let estimate = num / mean;
            mean = (mean + estimate) / 2.0;
        }

        return mean;
    }
    pub fn Factorial(n: u64) -> u64 {
        let mut ans = 1;

        for i in 1..=n {
            ans *= i;
        }

        return ans;
    }
    pub fn Pow(base: f64, exp: i64) -> f64 {
        if exp == 0 {
            return 1_f64;
        }

        let mut ans = 1.0;

        for _ in 1..=exp.abs() {
            ans *= base;
        }
        if exp < 0 {
            return 1.0 / ans;
        } else {
            return ans;
        }
    }
}

pub mod Fibonacci {
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

#[cfg(test)]
mod Tests {
    use super::*;
    #[test]
    fn TestPow() {
        assert_eq!(Arithmetic::Pow(4_f64, 4), 256_f64);
        assert_eq!(Arithmetic::Pow(17_f64, 0), 1_f64);
        assert_eq!(Arithmetic::Pow(2.5, 3), 15.625);
        assert_eq!(Arithmetic::Pow(16.0, -2), 0.00390625);
    }

    #[test]
    fn TestFactorial() {
        assert_eq!(Arithmetic::Factorial(5), 120);
    }

    #[test]
    fn TestSqrt() {
        assert_eq!(Arithmetic::Sqrt(5_f64, None).round(), 2_f64);
        assert_eq!(Arithmetic::Sqrt(64_f64, Some(10)), 8.0);
    }
}
