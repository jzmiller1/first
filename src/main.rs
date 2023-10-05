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

    let freqs = utils::frequency("12ABjBA1WROJEX(U@#X(@(#((@((@DKODJWOJEWOJWO");
    println!("{:?}", freqs);

    let probs = utils::freq_to_prob(&freqs);
    println!("{:?}", freqs);

    let entropy = utils::entropy(&probs);
    println!("Entropy is: {:?}", entropy);

    let tree = utils::huffman(probs);
    println!("{:#?}", tree);

    // Using Iterator::collect and Vec::join
    let keys: Vec<String> = freqs.keys().map(|&k| k.to_string()).collect();
    let keys_str = keys.join(" ");
    println!("{}", keys_str);

    // Using Iterator::for_each
    freqs.keys().for_each(|key| print!("{} ", key));
    println!();

}