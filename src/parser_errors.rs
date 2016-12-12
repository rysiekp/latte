use std::io::Write;

#[derive(Debug)]
pub enum ErrorType {
    OverflowError,
}

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

pub fn pretty_eof() {
    println_stderr!("err: unexpected EOF");
}

pub fn pretty_invalid(s: String, loc: usize)  {
    let (line_no, token_no) = count_line_and_pos(&s, loc);
    println_stderr!("err: {}:{}: invalid token '{}'", line_no, token_no, s.chars().nth(loc).unwrap());
}

pub fn pretty_unrecognized(s: String, token: &str, beg: usize, end: usize) {
    let (line_no, token_no) = count_line_and_pos(&s, beg);
    println_stderr!("err: {}:{}-{}: unexpected token '{}'", line_no, token_no, token_no + (end - beg), token);
}

pub fn pretty_user(s: String, err_type: ErrorType, err: String, loc: usize) {
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