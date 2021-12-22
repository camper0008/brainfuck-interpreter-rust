mod brainfuck;
mod context;
mod io;

fn main() {
    let code: String = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.".to_string();
    let mut ctx = context::Context {
        bracket_pairs: Vec::new(),
        stack: Vec::new(),
        stack_index: 0,
    };
    ctx.generate_bracket_pairs(code.clone());

    brainfuck::run(code.clone(), ctx)
}
