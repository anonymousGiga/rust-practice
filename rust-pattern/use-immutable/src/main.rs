fn main() {
    let mut data = vec![2, 1, 4, 10, 3, 5];
    data.sort();
    let data = data; // 进行重新绑定，data变为不可变的变量

    println!("{:?}", data[2]);

    // data.push(4); // error, data is immutable

    // 也可以使用如下使用嵌套块，和上面等价
    let data = {
        let mut data = vec![2, 1, 4, 10, 3, 5];
        data.sort();
        data
    };
    println!("{:?}", data[2]);
    // data.push(4); // error, data is immutable
}
