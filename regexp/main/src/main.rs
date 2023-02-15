use pcre2_lib;
use std::net;

fn main() {
    let pattern = r"(\d{4})([^\d\s]{3,11})(.|\n)";
    let subject = String::from("a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999");

    let result = pcre2_lib::match_data(pattern, &subject).unwrap();
    pcre2_lib::print_match_result(&result);

    let socket = net::UdpSocket::bind("127.0.0.1:9090").expect("couldn't bind to address");

    let _ = result
        .iter()
        .map(|s| {
            println!("send {:?}", s);
            socket
                .send_to(s.as_bytes(), "127.0.0.1:9091")
                .expect("couldn't send data");
        })
        .collect::<Vec<_>>();
}
