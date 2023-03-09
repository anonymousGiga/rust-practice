#[derive(Debug)]
pub struct StudentInfo1 {
    pub name: String,
    pub age: u32,
    pub number: u32,
}

// 第一种方案============================================
#[non_exhaustive]
#[derive(Debug)]
pub struct StudentInfo2 {
    pub name: String,
    pub age: u32,
    pub number: u32,
}

impl StudentInfo2 {
    pub fn set_student_info(name: String, age: u32, number: u32) -> Self {
        StudentInfo2 { name, age, number }
    }
}

// 第二种方案============================================
#[derive(Debug)]
pub struct StudentInfo3 {
    pub name: String,
    pub age: u32,
    pub number: u32,
    _b: (), //添加一个私有成员
}
