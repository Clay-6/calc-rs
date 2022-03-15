#![allow(non_snake_case)]

fn main() {
    Fibonacci::UpTo(1000);
}

fn Sqrt(num: f64, iters: u32) -> f64 {
    let mut mean = (num + 1.0) / 2.0;

    for _ in 0..iters {
        let estimate = num / mean;
        mean = (mean + estimate) / 2.0;
    }

    return mean;
}
fn Factorial(n: u64) -> u64 {
    let mut ans = 1;

    for i in 1..=n {
        ans *= i;
    }

    return ans;
}

mod Fibonacci {
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
