struct A {
    a: u8,
    b: Box<[u8;100]>,
}
struct B {
    a: u8,
    b: Box<u8>,
}
fn main() {
    let a = A {
        a: 1,
        b: Box::new([10;100]),
    };
    println!("size: {:?}", std::mem::size_of_val(&a));
    println!("size: {:?}", std::mem::size_of::<A>());
    println!("size: {:?}", std::mem::size_of::<B>());
}
