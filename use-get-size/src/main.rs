use alloy_primitives::{
    self, address, b256, bytes, fixed_bytes, hex, hex_literal, ruint, uint, Address, Bytes,
    FixedBytes, B256, I256, U256,
};

use get_size::GetSize;
// use hashbrown::{hash_map, hash_set, HashMap, HashSet};
use rand::{thread_rng, Fill, Rng};

use std::collections::{hash_map, HashMap};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct BundleState {
    pub contracts: HashMap<B256, Bytecode>,
}

/// State of the [`Bytecode`] analysis.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum BytecodeState {
    /// No analysis has been performed.
    Raw,
    /// The bytecode has been checked for validity.
    Checked,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Bytecode {
    pub bytecode: Bytes,
    pub state: BytecodeState,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct MyStruct {
    state: BundleState,
    others: HashMap<u8, Bytecode>,
}

#[derive(GetSize)]
struct MyHashMap(HashMap<u64, Data>);

fn main() {
    let mock_datas = generate_mock_datas(1_000_000);
    let mut sizes: Vec<usize> = Vec::new();
    sizes.push(0);
    sizes.push(10);
    sizes.push(100);
    sizes.push(1000);

    for size in (10_000..=1_000_000).step_by(10_000) {
        sizes.push(size);
    }

    let mut my_hashmap = MyHashMap(HashMap::new());
    let data = [100u8; 100];

    for size in sizes.iter() {
        for (key, val) in &mock_datas[..*size] {
            my_hashmap.0.insert(*key, *val);
        }

        println!("{:?},{:?}", size, my_hashmap.get_size());
        println!("cap =========== {:?}", my_hashmap.0.capacity() * 101);
        println!("len =========== {:?}", my_hashmap.0.len());
        println!("cap1 =========== {:?}", my_hashmap.0.capacity());
    }
}

#[derive(Clone, Copy, Debug, GetSize)]
pub struct Data {
    pub data: [u8; 100],
}

pub fn generate_mock_datas(len: usize) -> Vec<(u64, Data)> {
    let mut rng = thread_rng();
    let mut datas = Vec::with_capacity(len);

    for _ in 0..len {
        let mut data: [u8; 100] = [0; 100];
        data.try_fill(&mut rng).expect("filling data error");
        let val = Data { data };
        let key = rng.gen();
        datas.push((key, val));
    }

    datas
}
