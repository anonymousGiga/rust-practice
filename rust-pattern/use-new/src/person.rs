#[derive(Default)]
pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    // 使用new函数创建对象
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }

    pub fn print(&self) {
        println!("Name: {:?}", self.name);
        println!("Age: {:?}", self.age);
    }
}

// // 为Person实现Default trait
// impl Default for Person {
//     fn default() -> Self {
//         Self {
//             name: "".to_string(),
//             age: 0,
//         }
//     }
// }