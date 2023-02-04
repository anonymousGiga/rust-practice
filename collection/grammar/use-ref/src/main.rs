trait OrTrait {
    fn foo(self);
}

struct Check;
impl OrTrait for &Check {
    fn foo(self) {
        println!("A");
    }
}
impl OrTrait for &&&Check {
    fn foo(self) {
        println!("B");
    }
}

fn main() {
    let a = &Check;
    let b = &&Check;
    let c = &&&Check;
    let d = &&&&Check;
    a.foo();
    b.foo();
    c.foo();
    d.foo();
}
