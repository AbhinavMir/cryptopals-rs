use base64;

pub fn hex_to_base64(hex_string: &str) -> Result<String, base64::DecodeError> {
    let bytes = hex::decode(hex_string)?;
    Ok(base64::encode(&bytes))
}

pub fn check(){
    // print hello
    println!("Hello, world!");
}