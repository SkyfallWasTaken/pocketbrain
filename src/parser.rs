#[derive(Clone, Debug, PartialEq)]
pub enum Instr {
    Left,
    Right,
    IncrVal,
    DecrVal,
    Out,
    In,
    Loop(Vec<Self>),
}

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn consume_char(&mut self) -> Option<char> {
        let c = self.next_char();
        if let Some(c) = c {
            self.pos += c.len_utf8();
        }
        c
    }

    pub const fn from_input(input: String) -> Self {
        Self { pos: 0, input }
    }

    pub fn parse(&mut self) -> Vec<Instr> {
        let mut instrs = Vec::new();
        while let Some(r#char) = self.consume_char() {
            let instr = match r#char {
                '<' => Some(Instr::Left),
                '>' => Some(Instr::Right),
                '+' => Some(Instr::IncrVal),
                '-' => Some(Instr::DecrVal),
                ',' => Some(Instr::In),
                '.' => Some(Instr::Out),
                '[' => {
                    let mut inner = String::new();
                    let mut depth = 1; // Because of the first [ we've just matched on
                    while depth > 0 {
                        let c = self.consume_char();
                        match c {
                            Some(c) => {
                                match c {
                                    '[' => depth += 1,
                                    ']' => depth -= 1,
                                    _ => {}
                                }
                                inner.push(c);
                            }
                            _ => break,
                        }
                    }

                    let mut parser = Self::from_input(inner);
                    let instrs = parser.parse();
                    Some(Instr::Loop(instrs))
                }
                _ => None,
            };
            if let Some(instr) = instr {
                instrs.push(instr);
            }
        }
        instrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use Instr::*;

    #[test]
    fn nested_loops() {
        let input = "
            .
            [
              +
              [
                -
              ]
            ]
            +";
        let mut parser = Parser::from_input(input.to_string());
        let instrs = parser.parse();
        assert_eq!(
            instrs,
            vec![Out, Loop(vec![IncrVal, Loop(vec![DecrVal])]), IncrVal]
        )
    }
}
