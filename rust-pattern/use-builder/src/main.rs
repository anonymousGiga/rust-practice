#[derive(Debug)]
pub struct Teacher {
    name: String,
    age: String,
    gender: String,
    subject: String,
}

impl Teacher {
    pub fn builder() -> TeacherBuilder {
        TeacherBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct TeacherBuilder {
    name: String,
    age: String,
    gender: String,
    subject: String,
}

impl TeacherBuilder {
    pub fn new() -> Self {
        TeacherBuilder {
            name: "".to_string(),
            age: "".to_string(),
            gender: "".to_string(),
            subject: "".to_string(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn age(mut self, age: String) -> Self {
        self.age = age;
        self
    }

    pub fn gender(mut self, gender: String) -> Self {
        self.gender = gender;
        self
    }

    pub fn subject(mut self, subject: String) -> Self {
        self.subject = subject;
        self
    }

    pub fn build(self) -> Teacher {
        Teacher {
            name: self.name,
            age: self.age,
            gender: self.gender,
            subject: self.subject,
        }
    }
}

fn main() {
    let _math_teacher1 = TeacherBuilder::new()
        .name("Alice".to_string())
        .age("36".to_string())
        .gender("male".to_string())
        .subject("math".to_string())
        .build();

    let _math_teacher2 = TeacherBuilder::new()
        .name("Bob".to_string())
        .gender("male".to_string())
        .subject("math".to_string())
        .build();

    let _english_teacher = TeacherBuilder::new()
        .gender("female".to_string())
        .subject("english".to_string())
        .build();
}
