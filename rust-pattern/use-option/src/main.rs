fn main() {
    let a = Some("a string");
    let mut s1 = vec!["a", "b", "c"];
    s1.extend(a);
    println!("s1: {:?}", s1);

    println!("+++++++++++++++++++++++++++");

    let b = Some("b string");
    let s1 = vec!["d", "e", "f"];

    for s in s1.iter().chain(b.iter()) {
        println!("item: {}", s);
    }
}
