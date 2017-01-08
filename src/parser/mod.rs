pub mod parser;
#[macro_use]
pub mod parser_errors;

use ast::Program;
use utils::print_err;

pub fn parse(s: String) -> Program {
    match parser::parse_Program(remove_comments(&s).as_str()) {
        Ok(program) => program,
        Err(err) => {
            print_err(parser_errors::print_error(err, s));
            unreachable!()
        },
    }
}

fn remove_comments(s: &String) -> String {
    let mut last = ' ';
    let mut in_line_comment = false;
    let mut in_multi_comment = false;
    let mut in_quote = false;
    let mut res = String::new();
    for c in s.chars() {
        if !in_line_comment && !in_multi_comment && !in_quote {
            if last == '/' && c == '/' {
                in_line_comment = true;
            } else if last == '/' && c == '*' {
                in_multi_comment = true;
            } else if last == '/' {
                res = format!("{}{}{}", res, last, c);
            } else if c == '/' {

            } else if c == '\"' {
                in_quote = true;
                res = format!("{}{}", res, c);
            } else {
                res = format!("{}{}", res, c);
            }
        } else if in_line_comment {
            if c == '\n' {
                res = format!("{}\n", res);
                in_line_comment = false;
            }
        } else if in_multi_comment {
            if last == '*' && c == '/' {
                res = format!("{}\n", res);
                in_multi_comment = false;
                last = ' ';
                continue;
            }
        } else {
            if c == '\"' {
                in_quote = false;
            }
            res = format!("{}{}", res, c);
        }
        last = c;
    }
    res
}