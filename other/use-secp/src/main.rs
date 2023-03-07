fn main() {
    let signature: Vec<u8> = vec![
        62, 34, 142, 36, 101, 250, 182, 2, 144, 86, 124, 183, 103, 169, 164, 195, 38, 187, 234, 51,
        16, 15, 75, 126, 217, 79, 135, 239, 55, 139, 174, 133, 66, 181, 9, 44, 146, 81, 95, 152,
        117, 134, 19, 29, 154, 179, 182, 232, 72, 129, 129, 233, 223, 182, 203, 59, 171, 248, 131,
        193, 250, 32, 214, 178,
    ];

    let encoded_messages: Vec<u8> = vec![
        4, 12, 0, 209, 1, 66, 0, 0, 0, 48, 120, 100, 52, 51, 53, 57, 51, 99, 55, 49, 53, 102, 100,
        100, 51, 49, 99, 54, 49, 49, 52, 49, 97, 98, 100, 48, 52, 97, 57, 57, 102, 100, 54, 56, 50,
        50, 99, 56, 53, 53, 56, 56, 53, 52, 99, 99, 100, 101, 51, 57, 97, 53, 54, 56, 52, 101, 55,
        97, 53, 54, 100, 97, 50, 55, 100, 10, 0, 0, 0, 116, 116, 46, 116, 101, 115, 116, 110, 101,
        116, 128, 150, 152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ];

    let public_key: Vec<u8> = vec![
        2, 118, 94, 26, 241, 50, 154, 56, 35, 33, 23, 249, 126, 123, 168, 33, 93, 147, 187, 230,
        234, 44, 46, 23, 4, 151, 53, 235, 106, 186, 250, 119, 8,
    ];

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
