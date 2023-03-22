// 学生住校信息，如果是住校生，则需要提供其寝室号信息
mod example1 {
    pub enum ResidenceInformation {
        DayStudent,
        ResidentStudent,
    }

    pub struct Student {
        pub name: String,
        pub res_info: ResidenceInformation,
        pub bedroom_no: Option<u32>,
    }

    impl Student {
        pub fn print_name(&self) {
            println!("name is: {:?}", self.name);
        }

        pub fn print_res_info(&self) {
            if !self.bedroom_no.is_none() {
                println!("bed room number is: {:?}", self.bedroom_no);
            }
        }
    }
}

// 使用泛型实现上面的功能
mod example2 {
    pub struct DayStudent;
    pub struct ResidentStudent(pub u32);

    pub struct Student<T> {
        pub name: String,
        pub res_info: T,
    }

    impl<T> Student<T> {
        pub fn print_name(&self) {
            println!("name is: {:?}", self.name);
        }
    }

    impl Student<ResidentStudent> {
        pub fn print_res_info(&self) {
            println!("bed room number is: {:?}", self.res_info.0);
        }
    }
}

fn main() {
    let alice = example1::Student {
        name: "alice".to_string(),
        res_info: example1::ResidenceInformation::DayStudent,
        bedroom_no: None,
    };

    alice.print_name();
    alice.print_res_info();

    let bob = example1::Student {
        name: "bob".to_string(),
        res_info: example1::ResidenceInformation::ResidentStudent,
        bedroom_no: Some(1),
    };

    bob.print_name();
    bob.print_res_info();
    println!("++++++++++++++++++++++++++++++++++++++++++++");

    let charlie = example2::Student {
        name: "charlie".to_string(),
        res_info: example2::DayStudent,
    };
    charlie.print_name();
    println!("++++++++++++++++++++++++++++++++++++++++++++");

    let dave = example2::Student {
        name: "dave".to_string(),
        res_info: example2::ResidentStudent(1),
    };
    dave.print_name();
    dave.print_res_info();
}
