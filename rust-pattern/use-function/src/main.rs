// 命令式编程：
//      描述的是如何做某事
fn cmd_function() {
    let mut sum = 0;
    for i in 1..11 {
        sum += i;
    }
    println!("{}", sum);
}

// 函数式编程：
//      描述的是什么做什么
fn function_function() {
    println!("{}", (1..11).fold(0, |a, b| a + b));
}

fn main() {
    println!("命令式编程：");
    cmd_function();

    println!("函数式编程：");
    function_function();
}
