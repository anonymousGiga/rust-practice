use std::rc::Rc;

// #[derive(Clone)]
struct Sample;

fn foo<X>(_x: X) {
    match std::mem::size_of::<X>() {
        0 => println!("0"),
        _ => {
            // println!("{}", std::mem::size_of_val(&_x));
            println!("1")
        }
    }
}

fn main() {
    let sample = &Sample;
    let a = Sample;
    // println!("{}", std::mem::size_of_val(&a));
    // println!("{}", std::mem::size_of::<Sample>());
    // println!("{}", std::mem::size_of_val(&Sample));
    // println!("{}", std::mem::size_of_val(&&Sample));
    foo(sample);
    // println!("{}", std::mem::size_of_val(&((&Sample).clone())));
    foo(sample.clone());

    let sample2 = &();
    // println!("{}", std::mem::size_of_val(&()));
    // println!("{}", std::mem::size_of_val(&&()));
    foo(sample2);
    // println!("{}", std::mem::size_of_val(&((&()).clone())));
    // let a = &();
    // println!("{}", std::mem::size_of_val(&a));
    // println!("{}", std::mem::size_of_val(&(&()).clone()));
    foo(sample2.clone());

    let sample3 = Rc::new(());
    // println!("{}", std::mem::size_of_val(&sample3));
    // println!("{}", std::mem::size_of_val(&&sample3));
    foo(Rc::clone(&sample3));
    // println!("{}", std::mem::size_of_val(&sample3.clone()));
    foo(sample3.clone());
}
