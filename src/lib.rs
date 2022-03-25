#![allow(non_snake_case)]

pub mod Utils {
    pub fn ShowHelp() {
        println!("h - Show help");
        println!("The valid commands are:");
        println!("add <x> <y> - Adds <x> & <y> together");
        println!("sub <x> <y> - Does x - y");
        println!("mult <x> <y> - Multiplies <x> & <y>");
        println!("div <x> <y> - Performs x / y");
        println!("quot <x> <y> - Performs <x> DIV <y> (calculates the quotient)");
        println!("mod <x> <y> - Performs <x> MOD <y>");
        println!("pow <n> <e> - Calculate <n> to the power of <e> (defaults to 2)");
        println!(
            "sqrt <n> <i> - Calculate the square root of <n> with <i> iterations (defaults to 10)"
        );
        println!("fac <n> - Calculate the factorial of <n>");
        println!("fib-upto <n> - Show a fibonacci sequence up to <n>");
        println!("fib-oflen <n> - Show a fibonacci sequence of length <n>");
    }
}
pub mod Arithmetic {
    pub fn Multiply(x: f64, y: f64) -> f64 {
        x * y
    }
    pub fn Divide(x: f64, y: f64) -> f64 {
        x / y
    }
    pub fn Add(x: f64, y: f64) -> f64 {
        x + y
    }
    pub fn Subtract(x: f64, y: f64) -> f64 {
        x - y
    }
    pub fn Mod(x: f64, y: f64) -> f64 {
        x % y
    }
    pub fn Quotient(x: f64, y: f64) -> i64 {
        (x / y).floor() as i64
    }
    pub fn Sqrt(n: f64, iters: Option<u32>) -> f64 {
        let mut mean = (n + 1.0) / 2.0;
        let iters = match iters {
            Some(n) => n,
            None => 10,
        };

        for _ in 0..iters {
            let estimate = n / mean;
            mean = (mean + estimate) / 2.0;
        }

        return mean;
    }
    pub fn Factorial(n: i64) -> i128 {
        let mut ans: i128 = 1;

        for i in 1..=n.abs() as i128 {
            ans = match ans.checked_mul(i) {
                Some(num) => num,
                None => {
                    println!("Overflow occured. Reached up to:");
                    break;
                }
            }
        }

        if n % 2 != 0 {
            ans = -ans
        }

        return ans;
    }
    pub fn Pow(base: f64, exponent: Option<i64>) -> f64 {
        let exp = match exponent {
            Some(e) => e,
            None => 2,
        };

        if exp == 0 {
            return 1.0;
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
    pub fn QuadraticFormula(a: f64, b: f64, c: f64, pos: bool) -> f64 {
        let ans = match pos {
            true => (-b + Sqrt(Pow(b, Some(2)), Some(10)) - (4.0 * a * c)) / (2.0 * a),
            false => (-b - Sqrt(Pow(b, Some(2)), Some(10)) - (4.0 * a * c) / (2.0 * a)),
        };

        ans
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
    fn TestQuot() {
        assert_eq!(Arithmetic::Quotient(10.0, 5.0), 2);
        assert_eq!(Arithmetic::Quotient(17.0, 5.0), 3);
    }

    #[test]
    fn TestPow() {
        assert_eq!(Arithmetic::Pow(4.0, Some(4)), 256_f64);
        assert_eq!(Arithmetic::Pow(17.0, Some(0)), 1_f64);
        assert_eq!(Arithmetic::Pow(2.5, Some(3)), 15.625);
        assert_eq!(Arithmetic::Pow(16.0, Some(-2)), 0.00390625);
        assert_eq!(Arithmetic::Pow(5.0, None), 25.0);
    }

    #[test]
    fn TestFactorial() {
        assert_eq!(Arithmetic::Factorial(5), 120);
    }

    #[test]
    fn TestSqrt() {
        assert_eq!(Arithmetic::Sqrt(100.0, None), 10.0);
        assert_eq!(Arithmetic::Sqrt(64.0, Some(10)), 8.0);
    }
}
