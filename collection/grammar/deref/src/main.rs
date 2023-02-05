trait OrTrait {
    fn foo(self);
}

struct Check;

impl OrTrait for Check {
    fn foo(self) {
        println!("A");
    }
}

impl OrTrait for &Check {
    fn foo(self) {
        println!("B");
    }
}

impl OrTrait for &&&Check {
    fn foo(self) {
        println!("D");
    }
}

// 假定实现的trait的格式如下：
// impl OrTrait for U {
//     fn foo(self) {
//     ...
//     }
// }
// 则匹配规则：
// 1、先匹配T = U;
// 2、接下来匹配&T = U;
// 3、最后匹配&**T = U，此处可以多次解引用，即可以&**->&***->&****等;
// 4、再匹配不上，报错

fn main() {
    let a = Check;
    let b = &Check;
    let c = &&Check;
    let d = &&&Check;
    let e = &&&&Check;
    let f = &&&&&&&&Check;
    a.foo(); //规则1匹配成功，输出A
    b.foo(); //规则1匹配成功，输出B
    c.foo(); //规则1匹配失败; 规则2匹配，加上&按&(&&Check)->&&&Check匹配成功，输出D
    d.foo(); //规则1匹配成功，输出D
    e.foo(); //按照规则1匹配失败; 按照规则2匹配，加上&为&(&&&&Check)->&&&&&Check匹配不成功;按照规则3解引用，可以&**(&&&&)Check->&&&Check 匹配成功，所以输出D
    f.foo(); //按照规则1匹配失败; 按照规则2匹配，加上&为&(&&&&&&&&Check)->&&&&&&&&&Check匹配不成功;按照规则3解引用，可以&******(&&&&&&&&)Check->&&&Check 匹配成功，所以输出D
}
