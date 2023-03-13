use std::ops::Deref;

struct Foo;
impl Foo {
    fn do_something(&self) {
        println!("Do something");
    }
}

struct MyMutex<T> {
    flag: bool,
    data: T,
}

impl<T> MyMutex<T> {
    fn new(t: T) -> MyMutex<T> {
        MyMutex {
            flag: false,
            data: t,
        }
    }

    fn lock(&mut self) -> bool {
        if self.flag {
            println!("can't get lock");
            return false;
        }
        self.flag = true;
        println!("lock");
        true
    }
}

impl<T> Drop for MyMutex<T> {
    fn drop(&mut self) {
        self.flag = true;
        println!("unlock");
    }
}

impl<T> Deref for MyMutex<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn main() {
    {
        let mut m = MyMutex::new(Foo);
        println!("++++++++++");
        m.lock();
        m.lock(); //不能获取到锁
        println!("++++++++++");
        m.do_something();
    }
    println!("++++++++++");
}
