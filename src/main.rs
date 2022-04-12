mod brainfuck;
mod context;
mod io;

use crate::io::dump_ctx;

fn main() {
    let code = io::file_content_from_args();
    let mut ctx = context::Context::new(code);
    brainfuck::run(&mut ctx);
    if io::dump_flag_exists() {
        let res = dump_ctx(ctx);
        if res.is_err() {
            println!("error dumping context: {}", res.err().unwrap());
        }
    }
}
