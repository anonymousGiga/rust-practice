fn main() {
    let a = "A";
    const B: &str = "B";

    macro_rules! myMacro {
        () => {
            println!("{}{}", a, B);
        };
    }
    {
        let a = "C";
        const B: &str = "D";
        myMacro!();
    }
    println!("Hello, world!");
}
