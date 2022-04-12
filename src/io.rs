use crate::context::Context;
use std::env;
use std::fs::{write, File};
use std::io::stdin;
use std::io::Read;
use std::io::Result;
use std::path::PathBuf;

pub fn dump_flag_exists() -> bool {
    env::args().skip(1).any(|t| t == "--dump" || t == "-d")
}

fn file_absolute_path(relative_file_name: String) -> PathBuf {
    let mut wd = working_directory();
    wd.push(relative_file_name);
    wd
}

fn working_directory() -> PathBuf {
    env::current_dir().expect("unable to get current working directory")
}

fn file_name_from_args() -> String {
    let args = env::args().nth(1);
    if args.is_none() {
        println!("no input file given");
        std::process::exit(1);
    }
    return args.unwrap();
}

fn file_content(relative_file_name: String) -> Result<String> {
    let abs_name = file_absolute_path(relative_file_name.to_string());
    let mut buffer = String::new();
    File::open(abs_name)?.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn file_content_from_args() -> String {
    let file_name = file_name_from_args();

    match file_content(file_name.clone()) {
        Ok(c) => c,
        Err(e) => {
            println!("error reading from file '{}': {}", file_name, e);
            std::process::exit(1);
        }
    }
}

pub fn input_char() -> u8 {
    let mut buffer: [u8; 1] = [0];
    let mut take = stdin().take(1);
    (take.read(&mut buffer)).expect("unable to read from buffer");
    buffer[0]
}

fn html_body(ctx: &Context) -> String {
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

fn html_head(ctx: &Context) -> String {
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
        html_head(&ctx),
        html_body(&ctx),
    );

    write("dump.html", html)
}
