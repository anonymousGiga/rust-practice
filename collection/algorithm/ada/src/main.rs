fn is_super(c: u8) -> bool {
    match c {
        b'b' | b'f' | b'j' | b'p' | b'v' => true,
        _ => false,
    }
}

fn process(input: String) {
    if input.len() <= 1 {
        println!("{}", input);
        return;
    }

    let mut s = input.as_bytes().to_vec();
    let (mut begin, mut end): (usize, usize) = (0, input.len() - 1);

    loop {
        while begin < end && !is_super(s[begin]) {
            begin += 1;
        }

        while begin < end && !is_super(s[end]) {
            end -= 1;
        }

        println!("begin: {:?}, end: {:?}", begin, end);

        if begin < end {
            s.swap(begin, end);
            begin += 1;
            end -= 1;
        } else {
            break;
        }
    }

    println!("{}", std::str::from_utf8(&s).unwrap());
}

fn main() {
    let a = String::from("poobool");
    process(a);
    println!("Hello, world!");
}
