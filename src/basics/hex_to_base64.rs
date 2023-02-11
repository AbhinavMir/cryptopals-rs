use base64;

pub fn hex_to_base64() {
    let mut hex_string: String = String::new();
    // take user input
    println!("Enter a hex string:");
    std::io::stdin().read_line(&mut hex_string).expect("Failed to read line");
    // remove trailing newline
    hex_string = hex_string.trim().to_string();
    // String to &str
    let hex_string: &str = &hex_string;
    // convert hex to base64
    let bytes = hex::decode(hex_string).unwrap();
    let base64_string = base64::encode(&bytes);
    println!("Base64 string: {}", base64_string);
}