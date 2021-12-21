mod context;

fn brainfuck(code: String, mut ctx: context::Context) {
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
                //putChar(memory[memoryIndex]);
            }
            ',' => {
                //char input = getChar();
                //memory[memoryIndex] = input;
            }
            '>' => {
                if ctx.stack_index + 1 > ctx.stack.len() {
                    ctx.stack.push(0)
                }
            }
            '<' => {
                if ctx.stack_index == 0 {
                    ctx.stack_index = ctx.stack.len() - 1
                } else {
                    ctx.stack_index -= 1
                }
            }
            '[' => {
                if ctx.stack[ctx.stack_index] == 0 {
                    let pair = ctx
                        .get_bracket_pair(cursor)
                        .expect("could not find bracket pair");
                    cursor = pair.end;
                }
            }
            ']' => {
                if ctx.stack[ctx.stack_index] != 0 {
                    let pair = ctx
                        .get_bracket_pair(cursor)
                        .expect("could not find bracket pair");
                    cursor = pair.start;
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let code: String = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.".to_string();
    let mut ctx = context::Context {
        bracket_pairs: Vec::new(),
        stack: Vec::new(),
        stack_index: 0,
    };
    ctx.generate_bracket_pairs(code.clone());

    brainfuck(code.clone(), ctx)
}
