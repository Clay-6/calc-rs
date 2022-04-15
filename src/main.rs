use calculator::{arithmetic, evaluation::run, fibonacci};
use clap::Parser;

const PI: f64 = std::f64::consts::PI;
const EULER: f64 = std::f64::consts::E;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.operation.as_str() == "eval" {
        println!("{}", run(&args.a)?)
    } else {
        let a: f64 = match args.a.to_lowercase().as_str() {
            "pi" => PI,
            "e" => EULER,
            num => num.parse().unwrap(),
        };
        let b: f64 = match args.b.to_lowercase().as_str() {
            "pi" => PI,
            "e" => EULER,
            num => num.parse().unwrap(),
        };

        match args.operation.as_str() {
            "add" | "+" => println!("{}", a + b),

            "sub" | "-" => println!("{}", a - b),

            "mult" | "*" => println!("{}", a * b),

            "div" | "/" => println!("{}", a / b),

            "quot" => println!("{}", (a / b).floor()),

            "mod" | "%" => println!("{}", a % b),

            "pow" | "^" => println!("{}", arithmetic::pow(a, Some(b as i64))),

            "sqrt" => println!("{}", arithmetic::sqrt(a, b as u32)),

            "fac" | "!" => println!("{}", arithmetic::factorial(a as i64)),

            "quadratic" => {
                let c = match args.c.to_lowercase().as_str() {
                    "pi" => PI,
                    "e" => EULER,
                    num => num.parse().unwrap(),
                };
                let ans = arithmetic::quadratic_formula(a, b, c);
                if ans == (f64::NAN, f64::NAN) {
                    println!("No real number solutions exist");
                } else {
                    println!("x1 = {}, x2 = {}", ans.0, ans.1);
                }
            }

            "fib-upto" => fibonacci::up_to(a as u64),

            "fib-oflen" => fibonacci::of_length(a as u64),

            cmd => eprintln!("Invalid command \"{}\" Use -h or --help for help", cmd),
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The operation to perform
    ///
    /// Possible values (Symbol aliases shown in square brackets if available):
    /// add[+], sub[-], mult[*], div[/], quot, mod[%], pow[^], sqrt, fac[!], quadratic, fib-upto, fib-oflen, eval
    #[clap(short, long)]
    operation: String,
    /// The first number to use in the operation.
    /// Used as the string to evaluate if `eval` is the passed operation
    a: String,
    /// The second number to use in the operation
    #[clap(default_value = "0.0")]
    b: String,
    /// The third number to use in the operation. Only used in quadratic formula
    #[clap(default_value = "0.0")]
    c: String,
}
