use std::io;
use std::fs;


struct A<T> {
    a: T,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = "-";

    let (mut stdin_read, mut file_read);

    // 运行时确定具体的类型
    let readable: &mut dyn io::Read = if arg == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(arg)?;
        &mut file_read
    };

    // To do: read from readable

    let a: A<u8> = A{a:1u8};

    Ok(())
}