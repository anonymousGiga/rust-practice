enum SimpleError {
    IOError = 1,
    FileCorrupted = 2,
}

impl From<SimpleError> for libc::c_int {
    fn from(e: SimpleError) -> libc::c_int {
        (e as i8).into()
    }
}

enum SimpleError2 {
    IOError(std::io::Error),
    FileCorrupted(String),
}

impl From<SimpleError2> for libc::c_int {
    fn from(e: SimpleError2) -> libc::c_int {
        match e {
            SimpleError2::IOError(_) => 1,
            SimpleError2::FileCorrupted(_) => 2,
        }
    }
}

struct SimpleError3 {
    expected: char,
    line: u32,
    ch: u16,
}

#[repr(C)]
pub struct parse_error {
    pub expected: libc::c_char,
    pub line: u32,
    pub ch: u16,
}

impl From<SimpleError3> for parse_error {
    fn from(s: SimpleError3) -> parse_error {
        let SimpleError3 { expected, line, ch} = s;
        parse_error {
            expected: expected as libc::c_char,
            line,
            ch,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
