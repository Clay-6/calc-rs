use num_bigint::BigInt;

pub fn sqrt(n: f64, iters: u32) -> f64 {
    let mut mean = (n + 1.0) / 2.0;

    for _ in 0..iters {
        let estimate = n / mean;
        mean = (mean + estimate) / 2.0;
    }

    mean
}
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
pub fn pow(base: f64, exponent: Option<i64>) -> f64 {
    let exp = exponent.unwrap_or(2);

    if exp == 0 {
        return 1.0;
    }

    let mut ans = 1.0;

    for _ in 1..=exp.abs() {
        ans *= base;
    }

    if exp < 0 {
        1.0 / ans
    } else {
        ans
    }
}
pub fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return (f64::NAN, f64::NAN);
    }

    let x1 = (-b + sqrt(discriminant, 10)) / (2.0 * a);
    let x2 = (-b - sqrt(discriminant, 10)) / (2.0 * a);

    (x1, x2)
}
