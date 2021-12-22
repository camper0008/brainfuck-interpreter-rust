use std::env;
use std::fs::File;
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
