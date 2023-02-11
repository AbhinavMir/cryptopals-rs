mod basics;

fn main() {
    // take user input in hex_string variable
    let mut hex_string: String = String::new();
    println!("Enter a hex string:");
    std::io::stdin().read_line(&mut hex_string).expect("Failed to read line");
    basics::hex_to_base64::hex_to_base64(hex_string);
}
