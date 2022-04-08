pub mod arithmetic;
pub mod evaluation;
pub mod fibonacci;

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
