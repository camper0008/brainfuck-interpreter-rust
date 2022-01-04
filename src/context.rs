#[derive(PartialEq, Debug)]
pub enum TokenType {
    Increment,
    Decrement,
    IncrementPointer,
    DecrementPointer,
    Input,
    Output,
    LoopBegin,
    LoopEnd,
    Comment,
}
#[derive(Debug)]
pub struct Token {
    pub t: TokenType,
    pub v: usize,
}

fn generate_matching_brackets(tokens: &mut Vec<Token>) {
    let mut loop_indexes: Vec<usize> = Vec::new();
    let mut loop_tokens: Vec<&mut Token> = Vec::new();
    for (idx, mut token) in tokens.iter_mut().enumerate() {
        match token.t {
            TokenType::LoopBegin => {
                loop_indexes.push(idx);
                loop_tokens.push(token);
            }
            TokenType::LoopEnd => {
                let mut tok = loop_tokens.pop().expect("unopened bracket end");
                tok.v = idx;
                let idx = loop_indexes.pop().expect("unopened bracket end");
                token.v = idx;
            }
            _ => {}
        }
    }
    if loop_indexes.len() > 0 {
        panic!("unclosed bracket start");
    }
}

fn fold_duplicates(tokens: &mut Vec<Token>) {
    if tokens.len() < 2 {
        return;
    }
    let mut cursor = 0;
    while cursor < tokens.len() - 1 {
        match tokens[cursor].t {
            TokenType::Increment
            | TokenType::Decrement
            | TokenType::IncrementPointer
            | TokenType::DecrementPointer => {
                if tokens[cursor].t == tokens[cursor + 1].t {
                    tokens[cursor].v += tokens[cursor + 1].v;
                    tokens.remove(cursor + 1);
                } else {
                    cursor += 1;
                }
            }
            _ => cursor += 1,
        }
    }
}

fn tokenize(code: String) -> Vec<Token> {
    let mut res = Vec::new();
    for c in code.chars() {
        res.push(match c {
            '+' => Token {
                t: TokenType::Increment,
                v: 1,
            },
            '-' => Token {
                t: TokenType::Decrement,
                v: 1,
            },
            ',' => Token {
                t: TokenType::Input,
                v: 0,
            },
            '.' => Token {
                t: TokenType::Output,
                v: 0,
            },
            '>' => Token {
                t: TokenType::IncrementPointer,
                v: 1,
            },
            '<' => Token {
                t: TokenType::DecrementPointer,
                v: 1,
            },
            '[' => Token {
                t: TokenType::LoopBegin,
                v: 0,
            },
            ']' => Token {
                t: TokenType::LoopEnd,
                v: 0,
            },
            _ => Token {
                t: TokenType::Comment,
                v: 0,
            },
        })
    }

    res = res
        .into_iter()
        .filter(|token| token.t != TokenType::Comment)
        .collect();

    fold_duplicates(&mut res);
    generate_matching_brackets(&mut res);

    res
}

pub struct Context {
    pub cursor: usize,
    pub tokens: Vec<Token>,
    pub stack: Vec<u8>,
    pub stack_index: usize,
}

impl Context {
    pub fn new(code: String) -> Self {
        Self {
            cursor: 0,
            tokens: tokenize(code),
            stack: Vec::from([0]),
            stack_index: 0,
        }
    }
}
