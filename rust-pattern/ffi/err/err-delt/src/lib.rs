#![crate_type = "staticlib"]


// 惯常做法一
enum SimpleError {
    IOError = 1,
    FileCorrupted = 2,
}

impl From<SimpleError> for libc::c_int {
    fn from(e: SimpleError) -> libc::c_int {
        (e as i8).into()
    }
}

// 惯常做法二
enum SimpleError2 {
    IOError(String),
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

// 惯常做法三
struct SimpleError3 {
    expected: char,
    line: i32,
}

#[repr(C)]
pub struct parse_error {
    pub expected: libc::c_char,
    pub line: libc::c_int,
}

impl From<SimpleError3> for parse_error {
    fn from(s: SimpleError3) -> parse_error {
        let SimpleError3 { expected, line } = s;
        parse_error {
            expected: expected as libc::c_char,
            line: line as libc::c_int,
        }
    }
}

fn produce_err(t: u64) -> SimpleError {
    match t {
        1 => SimpleError::IOError,
        _ => SimpleError::FileCorrupted,
    }
}

fn produce_err2(t: u64) -> SimpleError2 {
    match t {
        1 => SimpleError2::IOError("IoError".to_string()),
        _ => SimpleError2::FileCorrupted("FileCorrupted".to_string()),
    }
}

fn produce_err3() -> SimpleError3 {
    SimpleError3 {
        expected: 'a',
        line: 1,
    }
}

#[no_mangle]
pub extern "C" fn return_err1(t: libc::c_int) -> libc::c_int {
    let err = produce_err(t as u64);
    libc::c_int::from(err)
}

#[no_mangle]
pub extern "C" fn return_err2(t: libc::c_int) -> *mut libc::c_char {
    let err = produce_err2(t as u64);
    let err_str = match err {
        SimpleError2::IOError(e) => format!("Io error: {:?}", e),
        SimpleError2::FileCorrupted(e) => format!("FileCorrupted: {:?}", e),
    };

    let c_error = unsafe {
        let malloc: *mut u8 = libc::malloc(err_str.len() + 1) as *mut _;
        if malloc.is_null() {
            return std::ptr::null_mut();
        }
        let src = err_str.as_bytes().as_ptr();
        std::ptr::copy_nonoverlapping(src, malloc, err_str.len());
        std::ptr::write(malloc.add(err_str.len()), 0);
        malloc as *mut libc::c_char
    };
    c_error
}

#[no_mangle]
pub extern "C" fn return_err3() -> parse_error {
    let err = produce_err3();
    parse_error::from(err)
}
