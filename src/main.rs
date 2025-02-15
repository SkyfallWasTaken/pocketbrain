mod machine;
mod parser;
use machine::Machine;

fn main() {
    let mut machine = Machine::from_input(include_str!("../test.bf").to_string());
    machine.execute();
    // println!("{:?}", instrs);
}
