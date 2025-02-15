use crate::parser::{Instr, Parser};
use std::io::{self, Read};

pub struct Machine {
    instrs: Vec<Instr>,
    // instr_ptr: usize,
    data_ptr: usize,
    data: Vec<u8>,
}

impl Machine {
    pub fn from_input(input: String) -> Self {
        let mut parser = Parser::from_input(input);
        let instrs = parser.parse();

        Self {
            instrs,
            // instr_ptr: 0,
            data_ptr: 0,
            data: vec![0; 30_000],
        }
    }

    pub fn step_instr(&mut self, instr: &Instr) {
        match instr {
            Instr::Left => self.data_ptr -= 1,
            Instr::Right => self.data_ptr += 1,

            Instr::DecrVal => self.data[self.data_ptr] -= 1,
            Instr::IncrVal => self.data[self.data_ptr] += 1,

            Instr::In => {
                let mut buf = [0; 1];
                io::stdin().read_exact(&mut buf).unwrap(); // TODO: don't panic here!
                let byte = buf[0];
                self.data[self.data_ptr] = byte;
            }
            Instr::Out => {
                let byte = self.data[self.data_ptr];
                println!("{}", byte as char);
            }

            Instr::Loop(loop_instrs) => {
                // dbg!(&loop_instrs);
                while self.data[self.data_ptr] != 0 {
                    for instr in loop_instrs {
                        // println!("stepping {:?}", instr);
                        self.step_instr(instr);
                    }
                }
            }
        }
    }

    pub fn execute(&mut self) {
        // TODO: don't clone here!
        for instr in &self.instrs.clone() {
            self.step_instr(instr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn move_data_ptr_left() {
        // N.B. requires right to be valid too (see below test)
        let mut machine = Machine::from_input(">><".into());
        machine.execute();
        assert_eq!(machine.data_ptr, 1);
    }

    #[test]
    fn move_data_ptr_right() {
        let mut machine = Machine::from_input(">>".into());
        machine.execute();
        assert_eq!(machine.data_ptr, 2);
    }

    #[test]
    fn incr_val() {
        let mut machine = Machine::from_input("+".into());
        machine.execute();
        assert_eq!(machine.data[0], 1);
    }

    #[test]
    fn decr_val() {
        // N.B. requires add to be valid (see above test)
        let mut machine = Machine::from_input("+++-".into());
        machine.execute();
        assert_eq!(machine.data[0], 2);
    }

    #[test]
    fn add_nums() {
        use Instr::*;
        let input = include_str!("../test_cases/add_nums.bf").to_string();
        let machine = Machine::from_input(input);
        assert_eq!(
            machine.instrs,
            vec![
                IncrVal,
                IncrVal,
                Right,
                IncrVal,
                IncrVal,
                IncrVal,
                IncrVal,
                IncrVal,
                Loop(vec![Left, IncrVal, Right, DecrVal]),
                IncrVal,
                IncrVal,
                IncrVal,
                IncrVal,
                IncrVal,
                IncrVal,
                IncrVal,
                IncrVal,
                Loop(vec![
                    Left, IncrVal, IncrVal, IncrVal, IncrVal, IncrVal, IncrVal, Right, DecrVal
                ]),
                Left,
                Out
            ]
        );
    }
}
