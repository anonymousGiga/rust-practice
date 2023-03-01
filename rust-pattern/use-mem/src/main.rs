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

fn convert_a_to_b2(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            name: mem::replace(name, String::new()),
        }
    }
}

// fn convert_a_to_b3(e: MyEnum) -> MyEnum {
//     if let MyEnum::A { name, x: 0 } = e {
//         return MyEnum::B{ name: name.clone()}
//     } else {
//         return e
//     }
// }

fn main() {
    let mut a = MyEnum::A {
        name: "A".to_string(),
        x: 0,
    };
    println!("Before Convert, a is {:?}", a);

    convert_a_to_b(&mut a);
    println!("After convert, a is {:?}", a);

    convert_a_to_b2(&mut a);
    println!("After convert2, a is {:?}", a);

    // let a =  convert_a_to_b3(a);
    // println!("After convert3, a is {:?}", a);
}
