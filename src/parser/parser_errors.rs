use lalrpop_util::ParseError;

#[derive(Debug)]
pub enum ErrorType {
    OverflowError,
}

type Error<'a> = ParseError<usize, (usize, &'a str), (ErrorType, String, usize)>;

pub fn print_error<'a>(err: Error<'a>, input: String) -> String {
    match err {
        ParseError::InvalidToken { location } => invalid(input, location),
        ParseError::UnrecognizedToken { token: None, .. } => eof(),
        ParseError::UnrecognizedToken { token: Some((beg, t, end)), .. } => unrecognized(input, t.1, beg, end),
        ParseError::User { error: (err_type, err, loc) } => user(input, err_type, err, loc),
        x => format!("{:?}", x),
    }
}

fn eof() -> String {
    format!("err: unexpected EOF")
}

fn invalid(s: String, loc: usize) -> String  {
    let (line_no, token_no) = count_line_and_pos(&s, loc);
    format!("err: {}:{}: invalid token '{}'", line_no, token_no, s.chars().nth(loc).unwrap())
}

fn unrecognized(s: String, token: &str, beg: usize, end: usize) -> String {
    let (line_no, token_no) = count_line_and_pos(&s, beg);
    format!("err: {}:{}-{}: unexpected token '{}'", line_no, token_no, token_no + (end - beg), token)
}

fn user(s: String, err_type: ErrorType, err: String, loc: usize) -> String {
    let (line_no, token_no) = count_line_and_pos(&s, loc);
    match err_type {
        ErrorType::OverflowError => format!("err: {}:{}: integer number too large: {} ", line_no, token_no, err),
    }
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