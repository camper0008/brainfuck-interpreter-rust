use crate::context::Context;
use crate::context::TokenType;
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

fn input(ctx: &mut Context) {
    ctx.stack[ctx.stack_index] = io::get_char();
}

fn output(ctx: &Context) {
    print!("{}", ctx.stack[ctx.stack_index] as char);
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

fn handle_loop_start(ctx: &mut Context) {
    if ctx.stack[ctx.stack_index] == 0 {
        ctx.cursor = ctx.tokens[ctx.cursor].v;
    }
}

fn handle_loop_end(ctx: &mut Context) {
    if ctx.stack[ctx.stack_index] != 0 {
        ctx.cursor = ctx.tokens[ctx.cursor].v;
    }
}

pub fn run(mut ctx: Context) {
    if ctx.stack.len() == 0 {
        ctx.stack.push(0);
    }

    while ctx.cursor < ctx.tokens.len() {
        match ctx.tokens[ctx.cursor].t {
            TokenType::Increment => incr(&mut ctx),
            TokenType::Decrement => decr(&mut ctx),
            TokenType::IncrementPointer => incr_ptr(&mut ctx),
            TokenType::DecrementPointer => decr_ptr(&mut ctx),
            TokenType::Input => input(&mut ctx),
            TokenType::Output => output(&ctx),
            TokenType::LoopBegin => handle_loop_start(&mut ctx),
            TokenType::LoopEnd => handle_loop_end(&mut ctx),
            TokenType::Comment => {}
        }
        ctx.cursor += 1;
    }
}
