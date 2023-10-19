use alloc::{Stats, TrackingAllocator};
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

#[derive(Clone, Copy)]
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

    let mut sizes: Vec<usize> = vec![0, 10, 100, 1_000];
    for size in (10_000..=1_000_000).step_by(10_000) {
        sizes.push(size);
    }

    println!("name,size,alloced,dealloced,diff");
    let alloc = TrackingAllocator;
    for size in sizes {
        run_and_track("hashmap", size, || {
            let mut m = HashMap::new_in(&alloc);

            for (key, val) in &large_pairs[..size] {
                m.insert(*key, *val);
            }

            m
        });
    }
}
