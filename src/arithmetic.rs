use anyhow::{anyhow, Result};
use num_bigint::BigInt;

/// Finds the square root of the given number
/// `n` using the Babylonian method in `iters` iterations
pub fn sqrt(n: f64, iters: u32) -> f64 {
    let mut mean = (n + 1.0) / 2.0;

    for _ in 0..iters {
        let estimate = n / mean;
        mean = (mean + estimate) / 2.0;
    }

    mean
}

/// Finds the factorial of `n`
pub fn factorial(n: i64) -> BigInt {
    let mut ans: BigInt = 1u64.into();

    for i in 1..=n.abs() {
        ans *= BigInt::from(i);
    }

    if n < 0 && n % 2 != 0 {
        ans = -ans
    }

    ans
}

/// Raises `n` to the power of `e`
pub fn pow(n: f64, e: i64) -> f64 {
    if e == 0 {
        return 1.0;
    }

    let mut ans = 1.0;

    for _ in 1..=e.abs() {
        ans *= n;
    }

    if e < 0 {
        1.0 / ans
    } else {
        ans
    }
}

/// Uses the quadratic formula to solve
/// `a`x^2 * `b`x + c
pub fn quadratic(a: f64, b: f64, c: f64) -> Result<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return Err(anyhow!("There are no real number roots for this quadratic"));
    }

    let x1 = (-b + sqrt(discriminant, 10)) / (2.0 * a);
    let x2 = (-b - sqrt(discriminant, 10)) / (2.0 * a);

    Ok((x1, x2))
}
