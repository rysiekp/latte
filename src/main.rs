extern crate lalrpop_util;

pub mod ast;
pub mod parser_errors;
mod parser;

fn main() {
    let s = "void f(int x) {\n\
                int x, y, z;\n\
                string x = 12;\n\
                while(c) {\n\
                    if (c) {\n\
                        printInt(2);\n\
                    }\n\
                }\n\
                return;\n\
             }\n\
             int main(bool x, string y) {\n\
                c++;\n\
                c--;\n\
                return c;\n\
             }";
    match parser::parse_Program(s) {
        Ok(x) => println!("{:?}", x),
        Err(err) => match err {
            lalrpop_util::ParseError::InvalidToken { location } => parser_errors::pretty_invalid(s.to_string(), location),
            lalrpop_util::ParseError::UnrecognizedToken { token: None, .. } => parser_errors::pretty_eof(),
            lalrpop_util::ParseError::UnrecognizedToken { token: Some((beg, t, end)), .. } => parser_errors::pretty_unrecognized(s.to_string(), t.1, beg, end),
            lalrpop_util::ParseError::User { error: (err_type, err, loc) } => parser_errors::pretty_user(s.to_string(), err_type, err, loc),
            x => println!("{:?}", x),
        },
    }
}
