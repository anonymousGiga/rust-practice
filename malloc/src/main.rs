mod heapsize;
mod types;

extern crate graphannis_malloc_size_of as malloc_size_of;
#[macro_use]
extern crate graphannis_malloc_size_of_derive as malloc_size_of_derive;

use std::mem::{size_of, size_of_val};

fn main() {
    types::test_size();

    let s = MyStruct { a: 1, b: 2, c: 3 };

    println!("size of s is: {:?}", size_of::<MyStruct>());
    println!("size of s is: {:?}", size_of_val(&s));
    println!("++++++++++++++++++++");
    types::test_mystruct2();
}

struct MyStruct {
    a: u32,
    b: u64,
    c: u128,
}
