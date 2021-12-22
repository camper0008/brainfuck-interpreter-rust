#[derive(Copy, Clone)]
pub struct BracketPair {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug)]
enum ErrorType {
    InvalidBracketPairError,
}

pub struct Context {
    pub bracket_pairs: Vec<BracketPair>,
    pub stack: Vec<u8>,
    pub stack_index: usize,
}

impl Context {
    fn create_bracket_pairs(&mut self, code: String, start: u32) -> Result<(), ErrorType> {
        let mut nested = 0;
        let mut pos = start;
        for i in code.chars().skip((1 + start).try_into().unwrap()) {
            pos += 1;
            if i == '[' {
                nested += 1;
            } else if i == ']' {
                if nested == 0 {
                    self.bracket_pairs.push(BracketPair {
                        start: start,
                        end: pos,
                    });
                    return Ok(());
                } else {
                    nested -= 1;
                }
            }
        }
        Err(ErrorType::InvalidBracketPairError)
    }
    pub fn generate_bracket_pairs(&mut self, code: String) {
        let mut pos = 0;
        for i in code.chars() {
            if i == '[' {
                self.create_bracket_pairs(code.clone(), pos)
                    .expect(format!("unclosed bracket start at character {}", pos).as_str());
            } else if i == ']' {
                self.get_bracket_pair(pos)
                    .expect(format!("unclosed bracket end at character {}", pos).as_str());
            }
            pos += 1;
        }
    }
    pub fn get_bracket_pair(&self, pos: u32) -> Option<BracketPair> {
        for i in 0..self.bracket_pairs.len() {
            if self.bracket_pairs[i].start == pos || self.bracket_pairs[i].end == pos {
                return Some(self.bracket_pairs[i]);
            }
        }
        None
    }
}
