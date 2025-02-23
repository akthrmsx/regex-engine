use automaton::Regex;
use clap::Parser;

/// String matcher by regular expression
#[derive(Debug, Parser)]
struct Args {
    /// Regular expression pattern
    pattern: String,
    /// Target text
    text: String,
}

fn main() {
    let args = Args::parse();

    match Regex::new(&args.pattern) {
        Ok(regex) => {
            if regex.matches(&args.text) {
                println!("Matched");
            } else {
                eprintln!("Unmatched")
            }
        }
        Err(err) => eprintln!("{}", err),
    }
}
