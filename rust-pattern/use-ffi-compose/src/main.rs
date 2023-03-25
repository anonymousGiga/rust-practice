struct MyVecWrapper<'a> {
    my_vec: Vec<&'a str>,
    iter_next: usize,
}

impl<'a> MyVecWrapper<'a> {
    pub fn first_item(&mut self) -> Option<&'a str> {
        self.iter_next = 0;
        self.next_item()
    }

    pub fn next_item(&mut self) -> Option<&'a str> {
        if let Some(next) = self.my_vec.get(self.iter_next) {
            self.iter_next += 1;
            Some(next)
        } else {
            None
        }
    }
}

fn main() {
    let a = "hello".to_string();
    let b = "world".to_string();
    let v = vec![&a, &b];

    // 模拟其它语言直接使用vector中的元素
    let first = v[0];
    println!("first string: {:?}", first);

    let second = v[1];
    println!("second string: {:?}", second);

    println!("+++++++++++++++++++++++++");
    // 下面的方式为：对象的所有可能的交互都被放入一个“封装器类型”中
    let a = "hello".to_string();
    let b = "world".to_string();
    let mut m_v = MyVecWrapper {
        my_vec: vec![&a, &b],
        iter_next: 0,
    };
    // 模拟其它语言获取该元素的引用
    let first = m_v.first_item().unwrap();
    println!("first string: {:?}", first);
}
