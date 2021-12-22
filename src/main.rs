mod context;

use std::io::stdin;
use std::io::Read;

fn get_char() -> u8 {
    let mut buffer: [u8; 1] = [0];
    let mut take = stdin().take(1);
    (take.read(&mut buffer)).expect("unable to read from buffer");
    buffer[0]
}

fn brainfuck(code: String, mut ctx: context::Context) {
    if ctx.stack.len() == 0 {
        ctx.stack.push(0);
    }

    let mut cursor: u32 = 0;
    while cursor < code.len().try_into().unwrap() {
        let c = code
            .chars()
            .nth(cursor.try_into().unwrap())
            .expect("could not get current character");
        match c {
            '+' => {
                if ctx.stack[ctx.stack_index] == u8::MAX {
                    ctx.stack[ctx.stack_index] = u8::MIN;
                } else {
                    ctx.stack[ctx.stack_index] += 1;
                }
            }
            '-' => {
                if ctx.stack[ctx.stack_index] == u8::MIN {
                    ctx.stack[ctx.stack_index] = u8::MAX;
                } else {
                    ctx.stack[ctx.stack_index] -= 1;
                }
            }
            '.' => {
                print!("{}", ctx.stack[ctx.stack_index] as char);
            }
            ',' => {
                ctx.stack[ctx.stack_index] = get_char();
            }
            '>' => {
                if ctx.stack_index + 2 > ctx.stack.len() {
                    ctx.stack.push(0)
                }
                ctx.stack_index += 1;
            }
            '<' => {
                if ctx.stack_index == 0 {
                    panic!("pointer went below 0");
                } else {
                    ctx.stack_index -= 1;
                }
            }
            '[' => {
                if ctx.stack[ctx.stack_index] == 0 {
                    let pair = ctx
                        .get_bracket_pair(cursor)
                        .expect("could not find bracket pair end");
                    cursor = pair.end;
                }
            }
            ']' => {
                if ctx.stack[ctx.stack_index] != 0 {
                    let pair = ctx
                        .get_bracket_pair(cursor)
                        .expect("could not find bracket pair start");
                    cursor = pair.start;
                }
            }
            _ => {}
        }
        cursor += 1;
    }
}

fn main() {
    let code: String = ",.".to_string();
    let mut ctx = context::Context {
        bracket_pairs: Vec::new(),
        stack: Vec::new(),
        stack_index: 0,
    };
    ctx.generate_bracket_pairs(code.clone());

    brainfuck(code.clone(), ctx)
}
