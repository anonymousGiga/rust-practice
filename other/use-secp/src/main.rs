fn main() {
    // let signature: Vec<u8> = vec![
    //     62, 34, 142, 36, 101, 250, 182, 2, 144, 86, 124, 183, 103, 169, 164, 195, 38, 187, 234, 51,
    //     16, 15, 75, 126, 217, 79, 135, 239, 55, 139, 174, 133, 66, 181, 9, 44, 146, 81, 95, 152,
    //     117, 134, 19, 29, 154, 179, 182, 232, 72, 129, 129, 233, 223, 182, 203, 59, 171, 248, 131,
    //     193, 250, 32, 214, 178,
    // ];
    let signature: Vec<u8> = vec![
        242, 85, 219, 225, 1, 128, 224, 110, 16, 158, 180, 246, 248, 247, 3, 134, 220, 216, 46, 44,
        93, 56, 142, 119, 236, 201, 189, 233, 239, 152, 64, 132, 59, 113, 73, 179, 253, 74, 48,
        162, 214, 157, 164, 195, 139, 249, 120, 25, 75, 67, 43, 76, 162, 152, 36, 86, 246, 3, 167,
        174, 162, 44, 112, 108,
    ];

    // let encoded_messages: Vec<u8> = vec![
    //     4, 12, 0, 209, 1, 66, 0, 0, 0, 48, 120, 100, 52, 51, 53, 57, 51, 99, 55, 49, 53, 102, 100,
    //     100, 51, 49, 99, 54, 49, 49, 52, 49, 97, 98, 100, 48, 52, 97, 57, 57, 102, 100, 54, 56, 50,
    //     50, 99, 56, 53, 53, 56, 56, 53, 52, 99, 99, 100, 101, 51, 57, 97, 53, 54, 56, 52, 101, 55,
    //     97, 53, 54, 100, 97, 50, 55, 100, 10, 0, 0, 0, 116, 116, 46, 116, 101, 115, 116, 110, 101,
    //     116, 128, 150, 152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0,
    // ];
    let encoded_messages: Vec<u8> = vec![
        4, 16, 0, 9, 2, 66, 0, 0, 0, 48, 120, 100, 52, 51, 53, 57, 51, 99, 55, 49, 53, 102, 100,
        100, 51, 49, 99, 54, 49, 49, 52, 49, 97, 98, 100, 48, 52, 97, 57, 57, 102, 100, 54, 56, 50,
        50, 99, 56, 53, 53, 56, 56, 53, 52, 99, 99, 100, 101, 51, 57, 97, 53, 54, 56, 52, 101, 55,
        97, 53, 54, 100, 97, 50, 55, 100, 24, 0, 0, 0, 97, 110, 100, 121, 45, 112, 97, 108, 108,
        101, 116, 45, 116, 101, 115, 116, 46, 116, 101, 115, 116, 110, 101, 116, 0, 0, 100, 167,
        179, 182, 224, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 110, 49, 217, 16, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];

    let public_key: Vec<u8> = vec![
        2, 118, 94, 26, 241, 50, 154, 56, 35, 33, 23, 249, 126, 123, 168, 33, 93, 147, 187, 230,
        234, 44, 46, 23, 4, 151, 53, 235, 106, 186, 250, 119, 8,
    ];
    // let public_key: Vec<u8> = vec![
    //     2, 92, 166, 45, 136, 166, 25, 34, 200, 8, 114, 136, 249, 236, 79, 56, 46, 123, 124, 133,
    //     35, 55, 102, 195, 33, 223, 214, 61, 250, 254, 133, 45, 20,
    // ];

    let signature =
        secp256k1::ecdsa::Signature::from_compact(&signature).expect("Invalid ecdsa signature.");
    println!("Signature: {:?}", signature);

    let message = hex::decode(sha256::digest(&encoded_messages[..])).expect("messages error");
    let message = secp256k1::Message::from_slice(&message).unwrap();
    println!("Message hash: {:?}", message);

    let public_key = secp256k1::PublicKey::from_slice(&public_key).unwrap();
    println!("Public key: {:?}", public_key);

    let secp = secp256k1::Secp256k1::new();
    if secp.verify_ecdsa(&message, &signature, &public_key).is_ok() {
        println!("Verify signature successed!");
    } else {
        println!("Verify signature failed!");
    }
}
