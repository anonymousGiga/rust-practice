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
    print_use_string1(&"content".to_string()); 

    let c = "content".to_string();
    print_use_str(&c);
    println!("c: {:?}", c);
    print_use_str("content");







    // let a = "content".to_string();
    // let b = &a;
    // let c = a.as_str();


    // println!("b: {:?}", b);
    // println!("c: {:?}", c);
}
