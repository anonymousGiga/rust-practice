fn d<T>(_f: T) {
    println!("d: {}", std::any::type_name::<T>());
    println!("size = {}", std::mem::size_of::<T>());
    match std::mem::size_of::<T>() {
        0 => println!("0"),
        1 => println!("1"),
        _ => println!("2"),
    }
}

fn a<T>(f: fn(T)) {
    println!("a: {}", std::any::type_name::<T>());
    println!("size = {}", std::mem::size_of::<T>());
    println!("a: {}", std::any::type_name::<fn(T)>());
    d(f);
}

fn b<T>(t: T) {
    println!("b: {}", std::any::type_name::<T>());
    println!("size = {}", std::mem::size_of::<T>());
    d(t);
}

fn main() {
    a(a::<u8>);
    println!("++++++++++++++++++");
    b(a::<u8>);
    println!("++++++++++++++++++");
    d(a::<u8>);
    println!("++++++++++++++++++");
    a(b::<u8>);
}