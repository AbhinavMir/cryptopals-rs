use base64;

pub fn hex_to_base64(hex_string: &str) {
    let bytes = hex::decode(hex_string).unwrap();
    let base64_string = base64::encode(&bytes);
    println!("Base64 string: {}", base64_string);
}