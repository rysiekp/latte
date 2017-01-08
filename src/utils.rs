use std::fs::File;
use std::env;
use std::process::*;
use std::io::Read;
use std::path::Path;
use std::io::Write;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

pub fn print_err(err: String) {
    println_stderr!("ERROR");
    println_stderr!("{}", err);
    exit(-1);
}

pub fn get_input() -> String {
    let mut to_parse = String::new();
    if let Some(ref filename) = env::args().nth(1) {
        if let Ok(mut input) = File::open(filename) {
            input.read_to_string(&mut to_parse).unwrap();
        } else {
            print_err(format!("Couldn't open file {}", filename));
        }
    } else {
        print_err(format!("Filename not given"));
    }
    to_parse
}

pub fn get_output_directory() -> String {
    let ref arg1 = env::args().nth(1).expect("Filename not given");
    let path = Path::new(arg1);
    let parent = path.parent().unwrap_or(Path::new("."));
    if parent.to_str().unwrap() == "" {
        String::from("./")
    } else {
        format!("{}/", parent.to_str().unwrap())
    }
}

pub fn get_output_filename(extension: &str) -> String {
    let no_ext = get_filename_no_ext();
    let parent = get_output_directory();
    format!("{}{}{}", parent, no_ext, extension)
}

pub fn get_filename_no_ext() -> String {
    let ref arg1 = env::args().nth(1).expect("Filename not given");
    let path = Path::new(arg1);
    let no_ext = path.file_stem().expect("Error opening file");
    format!("{}", no_ext.to_str().unwrap())
}