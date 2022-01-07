use crate::context::Context;
use std::env;
use std::fs::{write, File};
use std::io::stdin;
use std::io::Read;
use std::io::Result;
use std::path::PathBuf;

fn get_file_absolute_path(relative_file_name: String) -> PathBuf {
    let mut wd = get_working_directory();
    wd.push(relative_file_name);
    wd
}

fn get_working_directory() -> PathBuf {
    env::current_dir().expect("unable to get current working directory")
}

fn get_file_names_from_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // the command ran is always the first argument
    return args;
}

fn get_file_content(relative_file_name: String) -> Result<String> {
    let abs_name = get_file_absolute_path(relative_file_name.to_string());
    let mut buffer = String::new();
    File::open(abs_name)?.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn get_file_content_from_args() -> Vec<String> {
    let mut res = Vec::new();

    let file_names = get_file_names_from_args();
    for n in file_names.iter() {
        match get_file_content(n.to_string()) {
            Ok(c) => res.push(c),
            Err(e) => println!("error reading from file '{}': {}", n, e),
        }
    }

    res
}

pub fn get_char() -> u8 {
    let mut buffer: [u8; 1] = [0];
    let mut take = stdin().take(1);
    (take.read(&mut buffer)).expect("unable to read from buffer");
    buffer[0]
}

fn get_html_body(ctx: &Context) -> String {
    let mut elements = Vec::new();

    elements.push("<tbl-cnt><tbl>".to_string());
    elements
        .push("<clm><txt>idx</txt><txt>dec</txt><txt>char</txt><txt>hex</txt></clm>".to_string());
    for (i, t) in ctx.stack.iter().enumerate() {
        elements.push(format!("<clm id=\"clm-{}\">", i));
        elements.push(format!("<txt>{}</txt>", i));
        elements.push(format!("<txt>{}</txt>", t));
        elements.push(format!("<txt>{}</txt>", *t as char));
        elements.push(format!("<txt>{:x?}</txt>", t));
        elements.push("</clm>".to_string());
    }
    elements.push("</tbl></tbl-cnt>".to_string());
    elements.push(format!(
        "<info><txt>stack position: {} | <a href=\"#clm-{}\">jump to stack position</a></txt></info>",
        ctx.stack_index, ctx.stack_index
    ));

    elements
        .iter()
        .fold(String::new(), |acc, curr| format!("{}{}", acc, curr))
}

fn get_html_head(ctx: &Context) -> String {
    format!(
        "<style>
        body {{
            display: flex;
            background-color: #282828;
            margin: 0;
            flex-direction: column;
        }}
        tbl-cnt {{
            overflow-x: auto;
            box-sizing: border-box;
        }}
        tbl {{
            display: flex;
            background-color: #282828;
        }}
        clm {{
            display: flex;
            flex-direction: column;
            background-color: rgba(255, 255, 255, 0.15);
        }}
        clm:nth-child(odd) {{
            background-color: rgba(255, 255, 255, 0.0325);
        }}
        clm#clm-{} {{
            outline: 0.25rem solid #fff;
        }}
        info {{
            padding-top: 1rem;
        }}
        txt {{
            min-width: 3ch;
            min-height: 1.5em; /* because of default 1.5 lineheight */
            color: #fff;
            text-align: center;
            font-family: monospace;
            font-size: 1.25rem;
            padding: 1rem;
        }}
        a {{
            color: white;
        }}
        a:hover, a:focus-visible {{
            font-style: italic;
        }}
    </style>",
        ctx.stack_index
    )
}

pub fn dump_ctx(ctx: Context) -> Result<()> {
    let html = format!(
        "<html><head>{}</head><body>{}</body></html>",
        get_html_head(&ctx),
        get_html_body(&ctx),
    );

    write("dump.html", html)
}
