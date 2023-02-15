fn main() {
    let s1 = String::from("s1");
    // let a = "A";
    // let a = &s1;
    let a = s1;
    const B: &str = "B";

    // macro_rules! myMacro {
    //     () => {
    //         println!("{}:{}", a, B);
    //     };
    // }
    {
        let s2 = String::from("s2");
        // let a = "C";
        // let a = &s2;
        let a = s2;
        const B: &str = "D";
        macro_rules! myMacro {
            () => {
                println!("{}:{}", a, B);
            };
        }
        myMacro!();
    }
    println!("Hello, world!");
}
