use crate::parser::{Instr, Parser};

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

            Instr::In | Instr::Out => todo!("in/out not yet impl'd"),

            Instr::Loop(loop_instrs) => {
                for instr in loop_instrs {
                    self.step_instr(instr);
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
}
