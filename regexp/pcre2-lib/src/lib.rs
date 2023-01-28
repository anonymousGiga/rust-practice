use libc::c_int;
use std::ptr;

type Pcre2Uchar8 = u8;
type Pcre2Sptr8 = *const Pcre2Uchar8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcre2_real_compile_context_8 {
    _unused: [u8; 0],
}
pub type Pcre2CompileContext8 = pcre2_real_compile_context_8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcre2_real_code_8 {
    _unused: [u8; 0],
}
pub type Pcre2Code8 = pcre2_real_code_8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcre2_real_general_context_8 {
    _unused: [u8; 0],
}
pub type Pcre2GeneralContext8 = pcre2_real_general_context_8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcre2_real_match_context_8 {
    _unused: [u8; 0],
}
pub type Pcre2MatchContext8 = pcre2_real_match_context_8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcre2_real_match_data_8 {
    _unused: [u8; 0],
}
pub type Pcre2MatchData8 = pcre2_real_match_data_8;

#[link(name = "pcre2-8")]
extern "C" {
    pub fn pcre2_compile_8(
        arg1: Pcre2Sptr8,
        arg2: usize,
        arg3: u32,
        arg4: *mut c_int,
        arg5: *mut usize,
        arg6: *mut Pcre2CompileContext8,
    ) -> *mut Pcre2Code8;

    pub fn pcre2_match_data_create_from_pattern_8(
        arg1: *const Pcre2Code8,
        arg2: *mut Pcre2GeneralContext8,
    ) -> *mut Pcre2MatchData8;

    pub fn pcre2_match_8(
        arg1: *const Pcre2Code8,
        arg2: Pcre2Sptr8,
        arg3: usize,
        arg4: usize,
        arg5: u32,
        arg6: *mut Pcre2MatchData8,
        arg7: *mut Pcre2MatchContext8,
    ) -> c_int;

    pub fn pcre2_get_ovector_pointer_8(arg1: *mut Pcre2MatchData8) -> *mut usize;

    pub fn pcre2_match_data_free_8(arg1: *mut Pcre2MatchData8);
}

pub fn match_data(pattern: &str, subject: &str) -> Result<Vec<String>, &'static str> {
    let mut error_code = 0;
    let mut error_offset = 0;
    let mut start_offset = 0;
    let mut match_result: Vec<String> = Vec::new();

    unsafe {
        let re = pcre2_compile_8(
            pattern.as_ptr(),
            pattern.len(),
            0,
            &mut error_code,
            &mut error_offset,
            ptr::null_mut(),
        );
        if re.is_null() {
            return Err("Call pcre2_compile_8 error");
        }

        let match_data = pcre2_match_data_create_from_pattern_8(re, ptr::null_mut());
        if match_data.is_null() {
            return Err("Call pcre2_match_data_create_from_pattern_8 error");
        }

        loop {
            let rc = pcre2_match_8(
                re,
                subject.as_ptr(),
                subject.len(),
                start_offset,
                0,
                match_data,
                ptr::null_mut(),
            );

            if rc <= 0 {
                break;
            }

            let ovector = pcre2_get_ovector_pointer_8(match_data);
            for i in 0..rc {
                let (begin, end) = (
                    *ovector.offset((2 * i).try_into().unwrap()),
                    *ovector.offset((2 * i + 1).try_into().unwrap()),
                );

                // println!("{:?}, {:?}, {:?}", i, begin, end);

                if end - begin >= 8 {
                    //4个数字，结尾右边不为空，长度大于3
                    match_result.push(subject[begin + 4..end - 1].to_string());
                }

                start_offset = end as usize;
            }
        }
    }

    Ok(match_result)
}

pub fn print_match_result(result: &Vec<String>) {
    let _ = result
        .iter()
        .map(|s| {
            println!("{:?}", s);
        })
        .collect::<Vec<_>>();
}
