use automaton::Regex as Automaton;
use clap::{Parser, ValueEnum};
use virtual_machine::Regex as VirtualMachine;

/// String matcher by regular expression
#[derive(Debug, Parser)]
struct Args {
    /// Regular expression pattern
    pattern: String,
    /// Target text
    text: String,
    /// Engine type
    #[arg(value_enum, short = 't', long = "type", default_value_t = EnginType::Automaton)]
    engine_type: EnginType,
}

#[derive(Debug, Clone, ValueEnum)]
enum EnginType {
    /// Deterministic finite automaton
    #[value(name = "dfa", alias = "automaton")]
    Automaton,
    /// Virtual machine
    #[value(name = "vm", alias = "virtual-machine")]
    VirtualMachine,
}

fn main() {
    let args = Args::parse();

    match args.engine_type {
        EnginType::Automaton => match Automaton::new(&args.pattern) {
            Ok(regex) => {
                if regex.matches(&args.text) {
                    println!("Matched");
                } else {
                    eprintln!("Unmatched")
                }
            }
            Err(err) => eprintln!("{}", err),
        },
        EnginType::VirtualMachine => match VirtualMachine::new(&args.pattern) {
            Ok(regex) => {
                if regex.matches(&args.text) {
                    println!("Matched");
                } else {
                    eprintln!("Unmatched")
                }
            }
            Err(err) => eprintln!("{}", err),
        },
    }
}
