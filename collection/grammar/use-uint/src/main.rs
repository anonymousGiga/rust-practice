struct S(i32);

impl std::ops::BitAnd<S> for () {
    type Output = ();

    fn bitand(self, rhs: S) {
        println!("{}", rhs.0);
    }
}

fn main() {
    let f = || (() & S(1));
    let g = || () & S(2);
    let h = || ({} & S(3));
    // let h = || ({} & S(3)); 等价于下面的代码:
    // let h = || {
    //     ({
    //         println!("hello");
    //         ()
    //     } & S(3))
    // };

    let i = || {
        {}
        &S(4)
    };
    // 上面的代码等价于如下：
    // let i = || {
    //     &S(4)
    // };
    f();
    g();
    h();
    i();
}
