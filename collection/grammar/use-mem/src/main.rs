use std::{mem, collections::HashMap};

struct MyStruct {
    // field1: u32, // 4
    // field2: f64, // 8
    // field3: bool,// 1
    // str: String,
    map: HashMap<u128, u128>
}

pub enum AccountState {
    /// Before Spurious Dragon hardfork there was a difference between empty and not existing.
    /// And we are flaging it here.
    NotExisting,
    /// EVM touched this account. For newer hardfork this means it can be clearead/removed from state.
    Touched,
    /// EVM cleared storage of this account, mostly by selfdestruct, we don't ask database for storage slots
    /// and assume they are U256::ZERO
    StorageCleared,
    /// EVM didn't interacted with this account
    None,
}

fn main() {
    let my_struct_size = mem::size_of::<MyStruct>();
    println!("MyStruct的大小为 {} 字节", my_struct_size);

    let my_struct_size = mem::size_of::<AccountState>();
    println!("MyStruct的大小为 {} 字节", my_struct_size);

    let my_struct_size = mem::size_of::<Vec<u32>>();
    println!("MyStruct的大小为 {} 字节", my_struct_size);

}