struct Example {
    a: i32,
}

impl Drop for Example {
    fn drop(&mut self) {
        println!("Dropping the instance of Example with data : {}", self.a);
    }
}

fn get_value(a: i32) -> Example {
    Example { a }
}

fn main() {
    let a = get_value(1);
    println!("++++++++++++++++++++++");
}
