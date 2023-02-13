use std::cmp::Ordering;

// Define a function to convert hexadecimal string to byte array
fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
    let mut bytes = vec![];
    for i in (0..hex_string.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex_string[i..i+2], 16).unwrap();
        bytes.push(byte);
    }
    bytes
}

// Define a function to score plaintext based on character frequency
fn score_plaintext(plaintext: &[u8]) -> f64 {
    let mut score = 0.0;
    let mut frequency = [0; 26];
    for &byte in plaintext.iter() {
        if byte >= 65 && byte <= 90 {
            frequency[(byte - 65) as usize] += 1;
        } else if byte >= 97 && byte <= 122 {
            frequency[(byte - 97) as usize] += 1;
        }
    }

    // The character frequency table for English text
    let char_freq = [0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015,
                     0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749,
                     0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758,
                     0.00978, 0.02360, 0.00150, 0.01974, 0.00074];

    for i in 0..26 {
        score += (frequency[i] as f64 / plaintext.len() as f64 - char_freq[i]).abs();
    }
    score
}

// Define the main function
pub fn single_byte_xor() {
    let hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = hex_to_bytes(hex_string);
    let mut best_plaintext = vec![];
    let mut best_score = std::f64::INFINITY;
    let mut best_key = 0;
    for key in 0..255 {
        let plaintext = bytes.iter().map(|&b| b ^ key).collect::<Vec<u8>>();
        let score = score_plaintext(&plaintext);
        if score < best_score {
            best_plaintext = plaintext;
            best_score = score;
            best_key = key;
        }
    }
    println!("Key: {}", best_key);
    println!("Plaintext: {}", String::from_utf8_lossy(&best_plaintext));
}



