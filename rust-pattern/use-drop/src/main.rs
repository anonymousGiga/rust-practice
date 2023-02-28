#[derive(Debug)]
struct A(u8);
impl Drop for A {
    fn drop(&mut self) {
        println!("A exit");
        panic!("in drop");
        println!("A exit2");
    }
}

#[derive(Debug)]
struct B(u8);
impl Drop for B {
    fn drop(&mut self) {
        println!("B exit");
    }
}

fn main() {
    let a = A(1);
    {
        let b = B(1);
        println!("a: {:?}", a);
        println!("b: {:?}", b);
    }
    panic!("error");
}
