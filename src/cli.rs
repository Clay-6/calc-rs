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
    /// Add the given 2 numbers
    Add { a: f64, b: f64 },
    /// Subtract the given 2 numbers
    ///
    /// <A> - <B>
    Sub { a: f64, b: f64 },
    /// Multiply the given 2 numbers
    Mul { a: f64, b: f64 },
    /// Divide the given 2 numbers
    ///
    /// <A> / <B>
    Div { a: f64, b: f64 },
    /// Modulo of the given 2 numbers
    ///
    /// <A> % <B>
    Mod { a: f64, b: f64 },
    /// Quotient of the given 2 numbers
    ///
    /// Whole part of <A> / <B>
    Quot { a: f64, b: f64 },
    /// Raise a given number to a given power
    Pow { base: f64, exponent: Option<i64> },
    /// Calculate the square root of the given number in the
    /// given number of iterations
    ///
    /// Uses the babylonian method
    Sqrt { n: f64, iters: u32 },
    /// Calculate the factorial of the given number
    Fact { n: i64 },
    /// Solve the given quaratic equation
    Quadratic { a: f64, b: f64, c: f64 },
    /// Evaluate the given equation
    Eval { equation: String },
}
