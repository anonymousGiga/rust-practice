use std::rc::Rc;

fn main() {
    // 好的示范
    let a = Rc::new(1);
    let b = Rc::new(2);
    let c = Rc::new(3);

    let closure = {
        let b1 = b.clone();
        let c1 = c.as_ref();
        move || {
            let ret = *a + *b1 + *c1;
            println!("ret = {:?}", ret);
        }
    };

    closure();

    println!("++++++++++++++++++++++++++++++++");
    // 差的示范
    let a = Rc::new(1);
    let b = Rc::new(2);
    let c = Rc::new(3);
    let b_cloned = b.clone();
    let c_borrowed = c.as_ref();
    let closure = move || {
        let ret = *a + *b_cloned + *c_borrowed;
        println!("ret = {:?}", ret);
    };

    closure();
}
