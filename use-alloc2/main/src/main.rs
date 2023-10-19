// #![feature(allocator_api)]
use alloc::{Stats, TrackingAllocator, Vec};
// use alloc2::{Stats as Stats2, TrackingAllocator as TrackingAllocator2};
use hashbrown::HashMap;

use rand::{thread_rng, Fill, Rng};

pub fn run_and_track<T>(name: &str, size: usize, f: impl FnOnce() -> T) {
    alloc::reset();

    let t = f();

    let Stats {
        alloc,
        dealloc,
        diff,
    } = alloc::stats();
    println!("{name},{size},{alloc},{dealloc},{diff}");

    drop(t);
}

pub fn run_and_track2(name: &str, size: usize, allocator: &TrackingAllocator) {
    alloc::reset();

    let mut m1 = HashMap::new_in(allocator);
    let val = DummyData { data: [13; 100] };
    for i in 0..size {
        m1.insert(i, val);
    }

    let Stats {
        alloc,
        dealloc,
        diff,
    } = alloc::stats();
    println!("{name},{size},{alloc},{dealloc},{diff}");

    let mut m2 = HashMap::new_in(allocator);
    for i in 0..size {
        m2.insert(i, val);
    }

    let mut m3 = HashMap::new();
    let val = DummyData { data: [13; 100] };
    for i in 0..size {
        m3.insert(i, val);
    }

    let Stats {
        alloc,
        dealloc,
        diff,
    } = alloc::stats();
    println!("{name},{size},{alloc},{dealloc},{diff}");
    drop(m1);
    drop(m2);
    drop(m3);
}

pub fn run_and_track3(name: &str, size: usize, allocator: &TrackingAllocator) {
    alloc::reset();

    let mut v1 = Vec::new_in(allocator);
    let val = DummyData { data: [13; 100] };
    for _i in 0..size {
        v1.push(val)
    }

    let Stats {
        alloc,
        dealloc,
        diff,
    } = alloc::stats();
    println!("{name},{size},{alloc},{dealloc},{diff}");

    let mut v2 = Vec::new_in(allocator);
    for _i in 0..size {
        v2.push(val);
    }

    let mut v3 = Vec::new();
    for _i in 0..size {
        v3.push(val);
    }

    let Stats {
        alloc,
        dealloc,
        diff,
    } = alloc::stats();
    println!("{name},{size},{alloc},{dealloc},{diff}");

    for i in 0..v1.len() {
        v1.push(val);
        println!("i: {:?}, val: {:?}", i, v1[i]);
    }

    drop(v1);
    drop(v2);
    drop(v3);
}
#[derive(Clone, Copy, Debug)]
pub struct DummyData {
    pub data: [u8; 100],
}

pub fn generate_keys_values(len: usize) -> Vec<(u64, DummyData)> {
    let mut rng = thread_rng();

    let mut pairs = Vec::with_capacity(len);

    for _ in 0..len {
        let mut data: [u8; 100] = [0; 100];
        data.try_fill(&mut rng).expect("filling data should work");
        let val = DummyData { data };

        let key = rng.gen();

        pairs.push((key, val));
    }

    pairs
}

fn main() {
    let large_pairs = generate_keys_values(1_000_000);
    println!("generated data");
    println!();

    // let mut sizes: Vec<usize> = vec![0, 10, 100, 1_000];
    let mut sizes: Vec<usize> = Vec::new();
    sizes.push(0);
    sizes.push(10);
    sizes.push(100);
    sizes.push(1000);

    for size in (10_000..=1_000_000).step_by(10_000) {
        sizes.push(size);
    }

    println!("name,size,alloced,dealloced,diff");
    let alloc = TrackingAllocator;
    // for size in sizes {
    //     run_and_track("hashmap", size, || {
    //         let mut m = HashMap::new_in(&alloc);

    //         for (key, val) in &large_pairs[..size] {
    //             m.insert(*key, *val);
    //         }

    //         m
    //     });
    // }
    run_and_track2("hashmap", 1000, &alloc);

    println!("");
    println!("");
    println!("");
    println!("=============================================");
    println!("");

    // for size in sizes {
    //     run_and_track("vec-pair", size, || {
    //         let alloc = TrackingAllocator;
    //         let mut v = Vec::<DummyData, TrackingAllocator>::new_in(alloc);

    //         for (_key, val) in &large_pairs[..size] {
    //             v.push(*val);
    //         }

    //         v
    //     });
    // }
    run_and_track3("vector", 1000, &alloc);
}
