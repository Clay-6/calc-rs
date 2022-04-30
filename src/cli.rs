use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The operation to perform
    ///
    /// Possible values (Symbol aliases shown in square brackets if available):
    /// add[+], sub[-], mult[*], div[/], quot, mod[%], pow[^], sqrt, fac[!], quadratic, fib-upto, fib-oflen, eval
    #[clap(short, long)]
    pub operation: String,
    /// The first number to use in the operation.
    /// Used as the string to evaluate if `eval` is the passed operation
    pub a: String,
    /// The second number to use in the operation
    #[clap(default_value = "0.0")]
    pub b: String,
    /// The third number to use in the operation. Only used in quadratic formula
    #[clap(default_value = "0.0")]
    pub c: String,
}
