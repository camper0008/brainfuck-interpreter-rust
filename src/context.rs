pub struct BracketPair {
    pub start: usize,
    pub end: usize,
}

pub struct Context {
    pub bracket_pairs: Vec<BracketPair>,
    pub stack: Vec<u8>,
    pub stack_index: usize,
}

impl Context {
    fn create_bracket_pairs(&mut self, code: String, start: usize) {
        let mut nested = 0;
        let mut pos = start;
        for i in code.chars().skip(1 + start) {
            pos += 1;
            if i == '[' {
                nested += 1;
            } else if i == ']' {
                if nested == 0 {
                    self.bracket_pairs.push(BracketPair {
                        start: start,
                        end: pos,
                    });
                    return;
                } else {
                    nested -= 1;
                }
            }
        }
        panic!("unclosed bracket start character at '{}'", start)
    }
    pub fn generate_bracket_pairs(&mut self, code: String) {
        let mut pos = 0;
        for i in code.chars() {
            if i == '[' {
                self.create_bracket_pairs(code.clone(), pos);
            } else if i == ']' {
                self.get_bracket_start(pos);
            }
            pos += 1;
        }
    }
    pub fn get_bracket_start(&self, end: usize) -> usize {
        for v in &self.bracket_pairs {
            if v.end == end {
                return v.start;
            }
        }
        panic!("unopened bracket end character at '{}'", end)
    }
    pub fn get_bracket_end(&self, start: usize) -> usize {
        for v in &self.bracket_pairs {
            if v.start == start {
                return v.end;
            }
        }
        panic!("unclosed bracket start character at '{}'", start)
    }
}
