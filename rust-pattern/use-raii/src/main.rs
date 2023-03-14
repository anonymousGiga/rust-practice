use std::cell::RefCell;
use std::ops::Deref;

struct Foo;
impl Foo {
    fn do_something(&self) {
        println!("Do something");
    }
}

struct MyMutex<T> {
    flag: RefCell<bool>,
    data: T,
}

impl<T> MyMutex<T> {
    fn new(t: T) -> MyMutex<T> {
        MyMutex {
            flag: RefCell::new(false),
            data: t,
        }
    }

    fn lock(&self) -> Result<MyMutexGuard<T>, &'static str> {
        while *self.flag.borrow() {}
        *self.flag.borrow_mut() = true;
        MyMutexGuard::new(self)
    }

    fn try_lock(&self) -> Result<MyMutexGuard<T>, &'static str> {
        if *self.flag.borrow() == false {
            self.lock()
        } else {
            Err("Can't get lock")
        }
    }
}

impl<T> Drop for MyMutex<T> {
    fn drop(&mut self) {
        println!("MyMutex drop");
    }
}

struct MyMutexGuard<'a, T: 'a> {
    lock: &'a MyMutex<T>,
}

impl<'a, T> MyMutexGuard<'a, T> {
    fn new(lock: &'a MyMutex<T>) -> Result<MyMutexGuard<'a, T>, &'static str> {
        Ok(MyMutexGuard { lock })
    }
}

impl<'a, T> Deref for MyMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        println!("MyMutexGuard deref");
        &self.lock.data
    }
}

impl<'a, T> Drop for MyMutexGuard<'a, T> {
    fn drop(&mut self) {
        *self.lock.flag.borrow_mut() = false;
        println!("MyMutexGuard drop");
    }
}

fn main() {
    let m = MyMutex::new(Foo);
    // {
    println!("++++++++++");
    // let m1 = m.lock().unwrap();
    let m1 = m.lock();
    if let Err(e) = m.try_lock() {
        println!("Can't get lock, err: {:?}", e);
    }
    println!("++++++++++");
    // m1.do_something();
    // m1.unwrap().do_something();
    let m1 = m1.unwrap();
    m1.do_something();

    // }
    println!("++++++++++");
    let m2 = m.lock().unwrap();
    m2.do_something();
    println!("++++++++++");
}
