use crate::arithmetic;

const OPERATORS: [&str; 6] = ["+", "-", "/", "*", "%", "^"];
const PI: f64 = std::f64::consts::PI;
const E: f64 = std::f64::consts::E;

pub fn eval(equation: &str) -> f64 {
    let parts = equation
        .split_whitespace()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    let operators = get_operators(&parts);

    let mut ans = parts[0].parse().unwrap_or(0.0);

    for (idx, op) in operators {
        let next = match parts[idx + 1].as_str() {
            "PI" | "pi" => PI,
            "E" | "e" => E,
            n => n.parse().unwrap_or(0.0),
        };

        match op.as_str() {
            "+" => ans += next,
            "-" => ans -= next,
            "*" => ans *= next,
            "/" => ans /= next,
            "^" => ans = arithmetic::pow(ans, Some(next as i64)),
            "%" => ans %= next,
            _ => continue,
        }
    }

    ans
}

fn get_operators(parts: &[String]) -> Vec<(usize, String)> {
    let mut res = Vec::new();

    for (idx, part) in parts.iter().enumerate() {
        if OPERATORS.contains(&part.as_str()) {
            res.push((idx, part.clone()))
        }
    }

    res
}
