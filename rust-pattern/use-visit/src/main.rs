// 数据类型
mod ast {
    pub struct Name {
        pub value: String,
    }

    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}

// 抽象接口
mod visit {
    use crate::ast::*;

    pub trait Visitor<T> {
        fn visit_name(&mut self, n: &Name) -> T;
        fn visit_expr(&mut self, e: &Expr) -> T;
    }
}


// 具体的访问行为
mod interpreter{
    use crate::visit::*;
    use crate::ast::*;

    pub struct Interpreter;
    impl Visitor<i64> for Interpreter {
        fn visit_name(&mut self, _n: &Name) -> i64 { 1i64 }

        fn visit_expr(&mut self, e: &Expr) -> i64 {
            match *e {
                Expr::IntLit(n) => n,
                Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
                Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
            }
        }
    }
}

fn main() {
    use crate::visit::Visitor;
    let n = ast::Name{value: "alice".to_string()};
    let e = ast::Expr::IntLit(1i64);

    let mut inp = interpreter::Interpreter;
    let i = inp.visit_name(&n);
    println!("i: {:?}", i);

    let x = inp.visit_expr(&e);
    println!("x: {:?}", x);

}
