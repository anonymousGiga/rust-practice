struct A {
    // inner: Option<String>,
    inner: Option<u8>,
}

fn main() {
    let s = "Hello".to_string();
    // let a = A{inner: Some(s)};
    let a = A { inner: Some(0) };
    let b = match a.inner {
        Some(i) => i,
        // None => "s".to_string(),
        None => 0u8,
    };

    println!("{}", a.inner.unwrap());
    println!("{}", b);
    println!("Hello, world!");
}
