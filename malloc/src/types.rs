use super::heapsize;
use crate::malloc_size_of::MallocSizeOf;
use crate::malloc_size_of::MallocSizeOfOps;
use std::collections::HashMap;

#[derive(MallocSizeOf)]
struct MyStruct {
    a: Vec<u8>,
    b: HashMap<u8, HashMap<u8, u32>>,
    c: Option<Vec<u64>>,
}

pub fn test_size() {
    let mut my_struct = MyStruct {
        a: Vec::new(),
        b: HashMap::new(),
        c: None,
    };

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

#[derive(MallocSizeOf)]
pub struct Mystruct2 {
    data: HashMap<u64, Data>,
}

#[derive(Clone, Copy, Debug, MallocSizeOf)]
pub struct Data {
    pub data: [u8; 100],
}

pub fn test_mystruct2() {
    let mut ss = Mystruct2 {
        data: HashMap::new(),
    };
    let mut sizes: Vec<usize> = vec![0, 10, 100, 1_000];
    for size in (10_000..=1_000_000).step_by(10_000) {
        sizes.push(size);
    }

    let data = Data { data: [100; 100] };
    for size in sizes.iter() {
        for key in 0..*size {
            ss.data.insert(key as u64, data);
        }
        let mut ops = MallocSizeOfOps::new(heapsize::platform::usable_size, None, None);
        let size = ss.size_of(&mut ops);
        println!("size: {:?}", size);
    }
}
