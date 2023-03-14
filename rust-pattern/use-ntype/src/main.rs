//1、共享类型，同时实现精准控制:
// (1)personInfo类型共享studentInfo类型，同时又只提供name和id方法
// (2)PersonInfo实现Debug trait可以打印，PersonInfo则没有提供，变成了move语义
#[derive(Debug)]
struct StudentInfo {
    name: &'static str,
    id: &'static str,
    number: &'static str,
}

impl StudentInfo {
    fn new(name: &'static str, id: &'static str, number: &'static str) -> Self {
        StudentInfo {
            name,
            id,
            number: number,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn id(&self) -> &'static str {
        self.id
    }

    fn number(&self) -> &'static str {
        self.number
    }
}

struct PersonInfo(StudentInfo);

impl PersonInfo {
    fn new(s: StudentInfo) -> Self {
        PersonInfo(s)
    }

    fn name(&self) -> &'static str {
        self.0.name()
    }

    fn id(&self) -> &'static str {
        self.0.id()
    }
}

struct Miles(f64);
struct Kms(f64);
#[derive(Debug)]
struct Meter(u32);
fn main() {
    let s = StudentInfo::new("Alice", "123456", "001");
    let p = PersonInfo::new(s);
    println!("name: {:?}", p.name());
    println!("id: {:?}", p.id());

    let _m = Miles(10f64);
    let _kms = Kms(10f64);

    let m = Meter(1u32);
    let b = m;
    println!("b: {:?}", b);
    // println!("m: {:?}", m);
}
