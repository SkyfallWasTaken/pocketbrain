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

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> Option<char> {
        let c = self.next_char();
        if let Some(c) = c {
            self.pos += c.len_utf8();
        }
        c
    }

    fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && self.next_char().map_or(false, |c| test(c)) {
            if let Some(c) = self.consume_char() {
                result.push(c);
            }
        }
        result
    }

    pub fn from_input(input: String) -> Self {
        Parser { pos: 0, input }
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
                    let inner = self.consume_while(|c| c != ']');
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
