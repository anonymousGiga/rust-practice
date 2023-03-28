fn print_str(s: &str) {
    println!("Str is : {:?}", s);
}

fn main() {
    // {
    //     // 最开始的意图:
    //     let mut a = "Hello world".to_string();
    //     let b = &mut a;
    //     println!("Before modify, a: ");
    //     print_str(&a);

    //     println!("After modify, a: ");
    //     b.push('!');
    //     print_str(&b);
    // }

    // // 初学者经常会犯的错误，也是本节要说明的反面模式:
    // let mut a = "Hello world".to_string();
    // let b = &mut (a.clone()); //执行这行后，最开始的a和b已经不是同一个东西了
    // // let mut a1 = a.clone();
    // // let b = &mut a1; //执行这行后，最开始的a和b已经不是同一个东西了
    // println!("Before modify, a: ");
    // print_str(&a);

    // println!("After modify, a: ");
    // b.push('!');
    // print_str(&b);
    // print_str(&a);


    // 正确的做法: 更换一下打印代码的顺序
    // tips： 对于初学者来说，适当的更改一下所写代码的顺序，能很大程度上减少clone的使用
    
    let mut a = "Hello world".to_string();
    println!("Before modify, a: ");
    print_str(&a);

    let b = &mut a;
    println!("After modify, a: ");
    b.push('!');
    print_str(&b);
}
