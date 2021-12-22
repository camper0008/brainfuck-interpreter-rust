use crate::context::Context;
use crate::io;

fn incr(ctx: &mut Context) {
    if ctx.stack[ctx.stack_index] == u8::MAX {
        ctx.stack[ctx.stack_index] = u8::MIN;
    } else {
        ctx.stack[ctx.stack_index] += 1;
    }
}

fn decr(ctx: &mut Context) {
    if ctx.stack[ctx.stack_index] == u8::MIN {
        ctx.stack[ctx.stack_index] = u8::MAX;
    } else {
        ctx.stack[ctx.stack_index] -= 1;
    }
}

fn output(ctx: &Context) {
    print!("{}", ctx.stack[ctx.stack_index] as char);
}

fn input(ctx: &mut Context) {
    ctx.stack[ctx.stack_index] = io::get_char();
}

fn incr_ptr(ctx: &mut Context) {
    if ctx.stack_index + 1 >= ctx.stack.len() {
        ctx.stack.push(0)
    }
    ctx.stack_index += 1;
}

fn decr_ptr(ctx: &mut Context) {
    if ctx.stack_index == 0 {
        panic!("pointer went below 0");
    } else {
        ctx.stack_index -= 1;
    }
}

fn handle_loop_start(ctx: &mut Context, cursor: &mut u32) {
    if ctx.stack[ctx.stack_index] == 0 {
        let pair = ctx
            .get_bracket_pair(*cursor)
            .expect("could not find bracket pair end");
        *cursor = pair.end;
    }
}

fn handle_loop_end(ctx: &mut Context, cursor: &mut u32) {
    if ctx.stack[ctx.stack_index] != 0 {
        let pair = ctx
            .get_bracket_pair(*cursor)
            .expect("could not find bracket pair start");
        *cursor = pair.start;
    }
}

pub fn run(code: String, mut ctx: Context) {
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
            '+' => incr(&mut ctx),
            '-' => decr(&mut ctx),
            '.' => output(&ctx),
            ',' => input(&mut ctx),
            '>' => incr_ptr(&mut ctx),
            '<' => decr_ptr(&mut ctx),
            '[' => handle_loop_start(&mut ctx, &mut cursor),
            ']' => handle_loop_end(&mut ctx, &mut cursor),
            _ => {}
        }
        cursor += 1;
    }
}
