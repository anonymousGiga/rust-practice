fn say_hello1(name: &str) -> String {
    let mut result = "Hello ".to_owned();
    result.push_str(name);
    result.push('!');
    result
}

fn say_hello2(name: &str) -> String {
    format!("Hello {}!", name)
}

fn main() {
    let s = "world";
    println!("{:?}", say_hello1(s));
    println!("{:?}", say_hello2(s));
}
