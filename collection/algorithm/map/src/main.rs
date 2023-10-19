use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RevmMetricRecord {
    pub opcode_time: Option<HashMap<u8, (u64, u64)>>,
    pub host_time: u128,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub cache_misses_penalty: Vec<u64>,
}

fn main() {
    println!("Hello, world!");
}
