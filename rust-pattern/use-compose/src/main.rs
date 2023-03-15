struct A {
    a: u32,
    b: u32,
    c: u32,
}

fn foo(a: &mut A) -> &u32 {
    &a.b
}
fn bar(a: &mut A) -> u32 {
    a.a + a.b
}
fn baz(a: &mut A) {
    let x = foo(a);
    // let y = bar(a);  // 此行会报错
    println!("x: {:?}", x);
}

// ================================
// 组合结构体
// ================================
struct A1 {
    e: B,
    f: C,
}
struct B {
    b: u32,
}
struct C {
    a: u32,
    b: u32,
}

fn foo2(b: &mut B) -> &u32 {
    &b.b
}
fn bar2(c: &mut C) -> u32 {
    c.a + c.b
}

fn baz2(a: &mut A1) {
    let x = foo2(&mut a.e);
    let y = bar2(&mut a.f); //现在可以编译过了
    println!("x: {}", x);
    println!("y: {}", y);
}

fn main() {
    let mut a = A { a: 1, b: 2, c: 3 };
    baz(&mut a);

    let mut b = B { b: 2 };
    let mut c = C { a: 1, b: 3 };
    let mut a = A1 { e: b, f: c };
    baz2(&mut a);
}
