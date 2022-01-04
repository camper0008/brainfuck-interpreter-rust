mod brainfuck;
mod context;
mod io;

fn main() {
    for code in io::get_file_content_from_args().into_iter() {
        let ctx = context::Context::new(code);
        brainfuck::run(ctx)
    }
}
