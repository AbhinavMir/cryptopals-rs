mod basics;

fn main() {
    // take user input in hex_string variable
    let hex_string: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    basics::hex_to_base64::hex_to_base64(hex_string);
}
