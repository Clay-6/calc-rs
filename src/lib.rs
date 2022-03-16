#![allow(non_snake_case)]

pub mod Utils {
    pub fn ShowHelp() {
        println!("The valid arguments are:");
        println!("h - Show help");
        println!("pow [n] [e] - Calculate [n] to the power of [e]");
        println!("sqrt [n] [i] - Calculate the square root of [n] with [i] iterations");
        println!("fac [n] - Calculate the factorial of [n]");
        println!("fib-upto [n] - Show a fibonacci sequence up to [n]");
        println!("fib-oflen - Show a fibonacci sequence of length n");
    }
}
pub mod Numerical {
    pub fn Sqrt(num: f64, iters: u32) -> f64 {
        let mut mean = (num + 1.0) / 2.0;

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
    pub fn Pow(base: i64, exp: u64) -> i64 {
        if exp == 0 {
            return 1;
        }

        let mut ans = 1;

        for _ in 1..=exp {
            ans *= base;
        }

        return ans;
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
        assert_eq!(Numerical::Pow(4, 4), 256);
        assert_eq!(Numerical::Pow(17, 0), 1);
    }

    #[test]
    fn TestFact() {
        assert_eq!(Numerical::Factorial(5), 120);
    }

    #[test]
    fn TestSqrt() {
        assert_eq!(Numerical::Sqrt(5_f64, 10).round(), 2_f64);
    }
}
