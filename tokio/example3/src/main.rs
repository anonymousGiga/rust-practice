use std::sync::Arc;

struct Data {
    value: i32,
}

struct Struct1 {
    shared_data: Arc<Data>,
}

struct Struct2 {
    shared_data: Arc<Data>,
}

fn main() {
    let mut shared_data = Arc::new(Data { value: 42 });

    let struct1 = Struct1 {
        shared_data: Arc::clone(&shared_data),
    };

    let struct2 = Struct2 {
        shared_data: Arc::clone(&shared_data),
    };

    println!("Struct1: {}", struct1.shared_data.value);
    println!("Struct2: {}", struct2.shared_data.value);

    shared_data.value = 100;
    println!("Struct1: {}", struct1.shared_data.value);
    println!("Struct2: {}", struct2.shared_data.value);
}
