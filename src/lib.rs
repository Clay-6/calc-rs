pub mod arithmetic;
pub mod evaluation;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow() {
        assert_eq!(arithmetic::pow(4.0, 4), 256_f64);
        assert_eq!(arithmetic::pow(17.0, 0), 1_f64);
        assert_eq!(arithmetic::pow(2.5, 3), 15.625);
        assert_eq!(arithmetic::pow(16.0, -2), 0.00390625);
        assert_eq!(arithmetic::pow(5.0, 2), 25.0);
    }

    #[test]
    fn factorial() {
        assert_eq!(arithmetic::factorial(5), 120.into());
    }

    #[test]
    fn sqrt() {
        assert_eq!(arithmetic::sqrt(100.0, 10), 10.0);
        assert_eq!(arithmetic::sqrt(64.0, 10), 8.0);
    }

    #[test]
    fn eval() {
        assert_eq!(evaluation::run("1 + 1").unwrap(), 2.0);
        assert_eq!(evaluation::run("5 * 3").unwrap(), 15.0);
        assert_eq!(evaluation::run("18 / 2").unwrap(), 9.0);
        assert_eq!(evaluation::run("117 % 5").unwrap(), 2.0);
        assert_eq!(evaluation::run("18 ^ 2").unwrap(), 324.0);
        assert_eq!(evaluation::run("18 + -2").unwrap(), 16.0);
    }
}
