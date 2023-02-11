trait Sample {
    fn func(self);
}

impl<T> Sample for fn(T) {
    fn func(self) {
        print!("A");
    }
}

impl<T> Sample for fn(&T) {
    fn func(self) {
        print!("B");
    }
}
fn p(_: u8) {}
fn q(_: &u8) {}

fn main() {
    let first: fn(_) = p;
    let second: fn(_) = q;
    let third: fn(&_) = q;
    first.func();
    second.func();
    third.func();
}
