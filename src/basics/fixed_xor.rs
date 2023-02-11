pub fn fixed_xor() {
    // take two user buffers as input
    let mut buffer1: String = String::new();
    let mut buffer2: String = String::new();
    println!("Enter first buffer:");
    std::io::stdin().read_line(&mut buffer1).expect("Failed to read line");
    println!("Enter second buffer:");
    std::io::stdin().read_line(&mut buffer2).expect("Failed to read line");
    // remove trailing newline
    buffer1 = buffer1.trim().to_string();
    buffer2 = buffer2.trim().to_string();
    // String to &str
    let buffer1: &str = &buffer1;
    let buffer2: &str = &buffer2;
    // convert hex to base64
    let bytes1 = hex::decode(buffer1).unwrap();
    let bytes2 = hex::decode(buffer2).unwrap();
    // xor the two buffers
    let mut xor_result: Vec<u8> = Vec::new();
    for i in 0..bytes1.len() {
        xor_result.push(bytes1[i] ^ bytes2[i]);
    }
    // convert to hex
    let xor_result = hex::encode(xor_result);
    println!("XOR result: {}", xor_result);
}