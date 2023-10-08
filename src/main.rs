use std::collections::HashMap;

mod utils;
mod errors;
mod nodes;

fn main() {
    let freqs = utils::frequency("ABBA");
    println!("{:?}", freqs);

    let probs = utils::freq_to_prob(&freqs);
    println!("{:?}", probs);

    let entropy = utils::entropy(&probs);
    println!("Entropy is: {:?}", entropy);

    // Define your variable-length encodings for each symbol here
    let c: HashMap<char, &str> = [
        ('A', "0"),  // Suppose 'A' is encoded as "0"
        ('B', "1"),  // Suppose 'B' is encoded as "1"
        // Add more symbols and their encodings as needed
    ].iter().cloned().collect();

    let expected = utils::expected(&probs, &c);
    println!("Expected Length: {:?}", expected);

    let freqs = utils::frequency("1234567890ABjBA1WROJEX(U@#X(@(#((@((@DKODJWOJEWOJWOeeeeeeeeeeeeeeee aaaaaaaccchhh '{;#@ghjLKJ");
    println!("{:?}", freqs);

    let probs = utils::freq_to_prob(&freqs);
    println!("{:?}", freqs);

    let entropy = utils::entropy(&probs);
    println!("Entropy is: {:?}", entropy);

    let code = utils::huffman(&probs);
    println!("{:#?}", code);

    let expected = utils::expected(&probs, &code);
    println!("Expected Length: {:?}", expected);

    let encoded = utils::encoder("12 each", &code);
    match encoded {
        Ok(encoded_str) => {
            println!("Encoded: {:#?}", encoded_str);
            let decoded = utils::decoder(&encoded_str, &code);
            match decoded {
                Ok(value) => println!("Decoded: {:#?}", value),
                Err(e) => println!("An error occurred while decoding: {:?}", e),
            }
        },
        Err(e) => println!("An error occurred while encoding: {:?}", e),
    }

    let encoded = utils::encoder("12 Each", &code);
    match encoded {
        Ok(encoded_str) => {
            println!("Encoded: {:#?}", encoded_str);
            let decoded = utils::decoder(&encoded_str, &code);
            match decoded {
                Ok(value) => println!("Decoded: {:#?}", value),
                Err(e) => println!("An error occurred while decoding: {:?}", e),
            }
        },
        Err(e) => println!("An error occurred while encoding: {:?}", e),
    }

}