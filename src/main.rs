mod machine;
mod parser;
use clap::Parser;
use machine::Machine;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Brainfuck source file
    file: String,
}

fn main() {
    let args = Args::parse();
    let source = fs::read_to_string(args.file).expect("Failed to read the source file");

    let mut machine = Machine::from_input(source);
    machine.execute();
}
