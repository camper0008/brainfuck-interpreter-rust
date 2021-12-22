mod brainfuck;
mod context;
mod io;

fn main() {
    for code in io::get_file_content_from_args().iter() {
        let mut ctx = context::Context {
            bracket_pairs: Vec::new(),
            stack: Vec::new(),
            stack_index: 0,
        };
        ctx.generate_bracket_pairs(code.clone());
        brainfuck::run(code.to_string(), ctx)
    }
}
