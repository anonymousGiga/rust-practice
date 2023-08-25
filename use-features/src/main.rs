fn main() {
    #[cfg(feature = "my_feature")]
    {
        println!("This code is only active when 'my_feature' is enabled.");
    }

    println!("Hello, world!");
    #[cfg(feature = "my_feature")]
    println!("Hello, world!");
}
