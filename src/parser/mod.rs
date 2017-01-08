pub mod parser;
#[macro_use]
pub mod parser_errors;

use ast::Program;
use std::io::Write;

pub fn parse(s: String) -> Option<Program> {
    match parser::parse_Program(remove_comments(&s).as_str()) {
        Ok(program) => Some(program),
        Err(err) => {
            println_stderr!("ERROR");
            parser_errors::print_error(err, s);
            None
        },
    }
}

fn remove_comments(s: &String) -> String {
    let mut last = ' ';
    let mut in_line_comment = false;
    let mut in_multi_comment = false;
    let mut res = String::new();
    for c in s.chars() {
        if !in_line_comment && !in_multi_comment {
            if last == '/' && c == '/' {
                in_line_comment = true;
            } else if last == '/' && c == '*' {
                in_multi_comment = true;
            } else if last == '/' {
                res = format!("{}{}{}", res, last, c);
            } else if c == '/' {
                last = c;
                continue;
            } else {
                res = format!("{}{}", res, c);
            }
        } else if in_line_comment && !in_multi_comment {
            if c == '\n' {
                res = format!("{}\n", res);
                in_line_comment = false;
            }
        } else {
            if last == '*' && c == '/' {
                res = format!("{}\n", res);
                in_multi_comment = false;
                last = ' ';
                continue;
            }
        }
        last = c;
    }
    res
}