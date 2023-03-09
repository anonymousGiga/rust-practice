use student::{StudentInfo1, StudentInfo2, StudentInfo3};

fn main() {
    let alice = StudentInfo1 {
        name: "Alice".to_string(),
        age: 18,
        number: 1,
    };
    println!("Student info of alice: {:?}", alice);

    let bob = StudentInfo2::set_student_info("Bob".to_string(), 18, 2);
    println!("Student info of bob: {:?}", bob);

    // // error
    // let charlie = StudentInfo2 {name: "Charlie".to_string(), age: 18, number: 3};
    // println!("Student info of charlie: {:?}", charlie);

    // let StudentInfo2 {name, age, number} = bob; // error
    let StudentInfo2 {
        name, age, number, ..
    } = bob;
    println!("name: {:?}, age: {:?}, number: {:?}", name, age, number);

    // // error
    // let dave = StudentInfo3 {
    //     name: "Dave".to_string(),
    //     age: 18,
    //     number: 4,
    //     _b: (),
    // };
    // println!("Student info of dave: {:?}", dave);
}
