fn print_use_string(word: String) {
    println!("{:?}", word);
}

fn print_use_string1(word: &String) {
    println!("{:?}", word);
}

fn print_use_str(word: &str) {
    println!("{:?}", word);
}

fn main() {
    let a = "content".to_string();
    print_use_string(a);
    // println!("a: {:?}", a);   //This will error

    let b = "content".to_string();
    print_use_string1(&b);
    println!("b: {:?}", b);
    // print_use_string1("content");  //This will error

    let c = "content".to_string();
    print_use_str(&c);
    println!("c: {:?}", c);
    print_use_str("content");
}
