pub mod evaluation;

pub mod arithmetic {
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
}
pub mod fibonacci {
    pub fn up_to(n: u64) {
        let (mut a, mut b) = (0, 1);

        while a <= n {
            print!("{} ", a);
            (a, b) = (b, a + b);
        }
    }
    pub fn of_length(n: u64) {
        let (mut a, mut b) = (0, 1);

        for _ in 0..n {
            print!("{} ", a);
            (a, b) = (b, a + b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow() {
        assert_eq!(arithmetic::pow(4.0, Some(4)), 256_f64);
        assert_eq!(arithmetic::pow(17.0, Some(0)), 1_f64);
        assert_eq!(arithmetic::pow(2.5, Some(3)), 15.625);
        assert_eq!(arithmetic::pow(16.0, Some(-2)), 0.00390625);
        assert_eq!(arithmetic::pow(5.0, None), 25.0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(arithmetic::factorial(5), 120.into());
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(arithmetic::sqrt(100.0, 10), 10.0);
        assert_eq!(arithmetic::sqrt(64.0, 10), 8.0);
    }
}
