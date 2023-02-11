use base64;

pub fn hex_to_base64(hex_string: String) {
    // convert hex string to bytes
    let bytes = hex::decode(hex_string).unwrap();
    // convert bytes to base64
    let base64_string = base64::encode(bytes);
    // print base64 string
    println!("{}", base64_string);
}

pub fn check(){
    // print hello
    println!("Hello, world!");
}