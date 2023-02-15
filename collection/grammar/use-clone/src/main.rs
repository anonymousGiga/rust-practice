struct Bar;

#[derive(Clone)]
struct Foo;

fn print_size<X>(_x: X) {
    // println!("{}", std::any::type_name::<X>());
    println!("Size is: {}", std::mem::size_of::<X>());
}

fn main() {
    let bar = &Bar;
    print_size(bar);
    print_size(bar.clone()); // 等价于 print_size(Clone::clone(&bar));

    println!("+++++++++++++++++++");
    let foo = &Foo;
    print_size(foo);
    print_size(foo.clone()); // 等价于 print_size(Clone::clone(foo));
    // print_size(Clone::clone(&&Foo));
}
