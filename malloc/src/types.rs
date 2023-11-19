use super::heapsize;
use std::collections::HashMap;
use crate::malloc_size_of::MallocSizeOf;
use crate::malloc_size_of::MallocSizeOfOps;

#[derive(MallocSizeOf)]
struct MyStruct {
    a: Vec<u8>,
    b: HashMap<u8, HashMap<u8, u32>>,
    c: Option<Vec<u64>>
}

pub fn test_size() {
    let mut my_struct = MyStruct {
        a: Vec::new(),
        b: HashMap::new(),
        c: None,
    };
    let mut ops = MallocSizeOfOps::new(heapsize::platform::usable_size, None, None);
    let size = my_struct.size_of(&mut ops);
    println!("size: {:?}", size);

    for i in 0..16 {
        my_struct.a.push(i);
    }

    let mut ops = MallocSizeOfOps::new(heapsize::platform::usable_size, None, None);
    let size = my_struct.size_of(&mut ops);
    println!("size: {:?}", size);

    let mut b = HashMap::new();
    b.insert(1u8, 1u32);
    b.insert(2u8, 2u32);
    b.insert(3u8, 3u32);
    b.insert(4u8, 4u32);
    b.insert(5u8, 5u32);
    my_struct.b.insert(1u8, b);

    let v = vec![1, 2, 3, 4, 5];
    my_struct.c = Some(v);

    let mut ops = MallocSizeOfOps::new(heapsize::platform::usable_size, None, None);
    let size = my_struct.size_of(&mut ops);
    println!("size: {:?}", size);
}