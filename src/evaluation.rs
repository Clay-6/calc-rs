use crate::arithmetic;
use std::collections::VecDeque;

const PI: f64 = std::f64::consts::PI;
const E: f64 = std::f64::consts::E;

pub fn eval(equation: &str) -> f64 {
    let parts = equation
        .split_whitespace()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    let mut operators = get_operators(&parts);

    let mut ans = 0.0;

    while let Some((idx, op)) = operators.pop_front() {
        let next = match parts[idx + 1].as_str() {
            "PI" | "pi" => PI,
            "E" | "e" => E,
            n => n.parse().unwrap_or(0.0),
        };
        let prev = match parts[idx - 1].as_str() {
            "PI" | "pi" => PI,
            "E" | "e" => E,
            n => n.parse().unwrap(),
        };

        match op.as_str() {
            "+" => ans += next,
            "-" => ans -= next,
            "*" => ans *= next,
            "/" => ans /= next,
            "^" => ans += arithmetic::pow(prev, Some(next as i64)),
            "%" => ans %= next,
            _ => continue,
        }
    }

    ans
}

fn get_operators(parts: &[String]) -> VecDeque<(usize, String)> {
    let mut res = VecDeque::new();

    for (idx, part) in parts.iter().enumerate() {
        match part.as_str() {
            "+" => res.push_back((idx, part.clone())),
            "-" => res.push_back((idx, part.clone())),
            "*" => res.push_front((idx, part.clone())),
            "/" => res.push_front((idx, part.clone())),
            "^" => res.push_front((idx, part.clone())),
            "%" => res.push_front((idx, part.clone())),
            _ => continue,
        }
    }

    res
}
