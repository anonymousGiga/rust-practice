use std::ops::Deref;

struct Foo;
impl Foo {
    fn m(&self) {
        println!("Use foo's method!");
    }
}

struct Bar {
    f: Foo,
}

// // 不推荐的方式：滥用deref来模拟继承
// impl Deref for Bar {
//     type Target = Foo;
//     fn deref(&self) -> &Foo {
//         &self.f
//     }
// }

// 推荐的方式：显式的实现m方法
impl Bar {
    fn m(&self) {
        self.f.m();
    }
}

fn main() {
    let b = Bar { f: Foo {} };
    b.m(); 
    // (*b).m(); 
    // b.f.m();
    // // (*b) == b.f
}
