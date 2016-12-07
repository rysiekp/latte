extern crate lalrpop_util;

pub mod ast;
mod parser_errors;
mod parser;

fn main() {
    let s = "void f(int x) {\n\
                int x, y, z;\n\
                int x = 2;\n\
                while(c) {\n\
                    if (c) {\n\
                        printInt(2);\n\
                    }\n\
                }\n\
                return;\n\
             }\n\
             int main() {\n\
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
            x => println!("{:?}", x),
        },
    }
}
