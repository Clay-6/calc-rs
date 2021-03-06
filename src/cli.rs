use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The operation to perform
    #[clap(subcommand)]
    pub operation: Operation,
}

#[derive(Debug, Subcommand)]
pub enum Operation {
    /// Find the sum of the given numbers
    Add { operands: Vec<f64> },
    /// Subtract the given numbers from
    /// each other
    ///
    /// Subtracts in series (<A> - <B> - <C> ...)
    Sub { operands: Vec<f64> },
    /// Find the product of the given numbers
    Mul { operands: Vec<f64> },
    /// Divide the given numbers by each other
    ///
    /// Divides in series (<A> / <B> / <C> ...)
    Div { operands: Vec<f64> },
    /// Modulo of the given 2 numbers
    ///
    /// <A> % <B>
    Mod { a: f64, b: f64 },
    /// Quotient of the given 2 numbers
    ///
    /// Whole part of <A> / <B>
    Quot { a: f64, b: f64 },
    /// Raise a given number to a given power
    Pow { base: f64, exponent: i64 },
    /// Calculate the square root of the given number
    ///
    /// Uses the babylonian method
    Sqrt {
        n: f64,
        #[clap(short, long = "iterations", default_value_t = 10)]
        iters: u32,
    },
    /// Calculate the factorial of the given number
    Fact { n: u64 },
    /// Solve the given quaratic equation
    Quadratic { a: f64, b: f64, c: f64 },
    /// Find the sine of <X> in radians
    Sin { x: f64 },
    /// Find the cosine of <X> in radians
    Cos { x: f64 },
    /// Find the tangent of <X> in radians
    Tan { x: f64 },
    /// Evaluate the given equation
    Eval { equation: String },
}
