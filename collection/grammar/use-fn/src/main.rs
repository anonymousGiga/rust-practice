fn d<T>(_f: T) {
    println!("a: {}", std::any::type_name::<T>());
    match std::mem::size_of::<T>() {
        0 => println!("0"),
        1 => println!("1"),
        _ => println!("2"),
    }
}

fn a<T>(f: fn(T)) {
    println!("a: {}", std::any::type_name::<T>());
    d(f);
}

fn main() {
    a(a::<u8>);
    d(a::<u8>);
}