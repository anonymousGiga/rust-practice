use std::mem;
#[derive(Debug)]
enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

fn convert_a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            name: mem::take(name),
        }
    }
}

fn modify_name_for_my_enum(e: &mut MyEnum, new_name: String) {
    *e = match e {
        MyEnum::A { name, x } => {
            _ = mem::replace(name, new_name);
            MyEnum::A {
                name: name.to_string(),
                x: *x,
            }
        }
        MyEnum::B { name } => {
            _ = mem::replace(name, new_name);
            MyEnum::B {
                name: name.to_string(),
            }
        }
    }
}

fn main() {
    let mut a = MyEnum::A {
        name: "A".to_string(),
        x: 0,
    };
    println!("Before Convert, a is {:?}", a);

    convert_a_to_b(&mut a);
    println!("After convert, a is {:?}", a);

    modify_name_for_my_enum(&mut a, "b".to_string());
    println!("After modify name, a is {:?}", a);
}
