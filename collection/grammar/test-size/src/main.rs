use std::mem;

#[derive(Clone)]
struct Foo; 

#[derive(Clone)]
struct Bar(u8);

fn main() {
    let a = Foo;
    let b = &Foo;
    println!("size = {}", mem::size_of_val(&a)); 
    println!("size = {}", mem::size_of_val(&b)); 
    println!("size = {}", mem::size_of_val(&a.clone())); 
    println!("size = {}", mem::size_of_val(&(b.clone()))); 

    println!("-----------------------------------------");

    let c = Bar(6);
    let d = &Bar(7);
    println!("size = {}", mem::size_of_val(&c)); 
    println!("size = {}", mem::size_of_val(&d)); 
    println!("size = {}", mem::size_of_val(&c.clone())); 
    println!("size = {}", mem::size_of_val(&(d.clone()))); 
}