use lalrpop_util::ParseError;
use std::io::Write;

#[derive(Debug)]
pub enum ErrorType {
    OverflowError,
}

type Error<'a> = ParseError<usize, (usize, &'a str), (ErrorType, String, usize)>;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

pub fn print_error<'a>(err: Error<'a>, input: &str) {
    match err {
        ParseError::InvalidToken { location } => invalid(input.to_string(), location),
        ParseError::UnrecognizedToken { token: None, .. } => eof(),
        ParseError::UnrecognizedToken { token: Some((beg, t, end)), .. } => unrecognized(input.to_string(), t.1, beg, end),
        ParseError::User { error: (err_type, err, loc) } => user(input.to_string(), err_type, err, loc),
        x => println!("{:?}", x),
    }
}

fn eof() {
    println_stderr!("err: unexpected EOF");
}

fn invalid(s: String, loc: usize)  {
    let (line_no, token_no) = count_line_and_pos(&s, loc);
    println_stderr!("err: {}:{}: invalid token '{}'", line_no, token_no, s.chars().nth(loc).unwrap());
}

fn unrecognized(s: String, token: &str, beg: usize, end: usize) {
    let (line_no, token_no) = count_line_and_pos(&s, beg);
    println_stderr!("err: {}:{}-{}: unexpected token '{}'", line_no, token_no, token_no + (end - beg), token);
}

fn user(s: String, err_type: ErrorType, err: String, loc: usize) {
    let (line_no, token_no) = count_line_and_pos(&s, loc);
    match err_type {
        ErrorType::OverflowError => println_stderr!("err: {}:{}: integer number too large: {} ", line_no, token_no, err),
    };
}

fn count_line_and_pos(s: &String, loc: usize) -> (usize, usize) {
    let mut line_no = 1;
    let mut token_no = 1;
    for (i, c) in s.chars().enumerate() {
        if c == '\n' {
            line_no += 1;
            token_no = 0;
        }
        if loc == i {
            break;
        }
        token_no += 1;
    };

    (line_no, token_no)
}