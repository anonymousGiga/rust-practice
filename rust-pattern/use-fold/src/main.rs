// 数据集
mod collection {
    #[derive(Debug)]
    pub enum Data1 {
        _Number(u32),
        _Name(String),
        _Other,
    }

    #[derive(Debug)]
    pub struct Data2 {
        pub name: String,
    }
}

// 抽象folder接口
mod fold {
    use crate::collection::*;

    pub trait Folder {
        fn fold_data1(&mut self, data: Box<Data1>) -> Box<Data1> {
            data
        }

        fn fold_data2(&mut self, data: Box<Data2>) -> Box<Data2> {
            data
        }
    }
}

// 重新实现新的folder
mod rename {
    use crate::collection::*;
    use crate::fold::*;

    pub struct Renamer;
    impl Folder for Renamer {
        fn fold_data1(&mut self, data: Box<Data1>) -> Box<Data1> {
            match *data {
                Data1::_Name(_) => Box::new(Data1::_Name("foo".to_string())),
                _ => data,
            }
        }

        fn fold_data2(&mut self, _data: Box<Data2>) -> Box<Data2> {
            Box::new(Data2 {
                name: "foo".to_string(),
            })
        }
    }
}

fn main() {
    use collection::*;
    use fold::*;
    use rename::*;

    let mut renamer = Renamer;

    let d1 = Box::new(Data1::_Name("alice".to_string()));
    println!("Before fold, d1: {:?}", d1);
    let d1 = renamer.fold_data1(d1);
    println!("After fold, d1: {:?}", d1);

    let d2 = Box::new(Data2 {
        name: "alice".to_string(),
    });
    println!("Before fold, d2: {:?}", d2);
    let d2 = renamer.fold_data2(d2);
    println!("After fold, d2: {:?}", d2);
}
