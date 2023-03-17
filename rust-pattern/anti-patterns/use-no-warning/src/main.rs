// 不推荐的方式: 拒绝所有警告
#![deny(warnings)]
fn main() {
    let a = "hello";
    println!("Hello, world!");
}

// 推荐的方式： 明确制定拒绝的lint
#![deny(unused_variables)]
fn main() {
    let a = "hello";
    println!("Hello, world!");
}
