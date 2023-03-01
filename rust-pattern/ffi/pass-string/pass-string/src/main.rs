extern "C" {
    fn set_err(message: *const libc::c_char);
}

// 正确示范
fn report_error_to_ffi<S: Into<String>>(err: S) -> Result<(), std::ffi::NulError> {
    let c_err = std::ffi::CString::new(err.into())?;

    unsafe {
        set_err(c_err.as_ptr());
    }

    Ok(())
}

// 错误示范
fn report_error<S: Into<String>>(err: S) -> Result<(), std::ffi::NulError> {
    unsafe {
        // SAFETY: whoops, this contains a dangling pointer!
        set_err(std::ffi::CString::new(err.into())?.as_ptr());
    }
    Ok(())
}

fn main() {
    let err = "some error";
    let _ = report_error_to_ffi(err);
    let _ = report_error(err);
}
