use std::collections::{BinaryHeap, HashMap};

use crate::nodes::{Node, NodeKind};
use crate::errors::SymbolMappingError;


/// Calculates the frequency of each character in a given string.
///
/// # Arguments
///
/// * `s` - A string slice that holds the text you wish to analyze.
///
/// # Returns
///
/// Returns a `HashMap` where the keys are the unique characters found in the string,
/// and the values are the frequencies of these characters.
///
/// # Examples
///
/// ```
/// let result = frequency("ABBA");
/// assert_eq!(result, [('A', 2), ('B', 2)]);
/// ```
pub fn frequency(s: &str) -> HashMap<char, i32> {
    let mut d = HashMap::new();
    for symbol in s.chars() {
        *d.entry(symbol).or_insert(0) += 1;
    }
    d
}

/// Converts the frequencies of each character in a given `HashMap` to probabilities.
///
/// # Arguments
///
/// * `f` - A reference to a `HashMap` containing character frequencies.
///
/// # Returns
///
/// Returns a new `HashMap` where the keys are the unique characters, and the values are probabilities.
///
/// # Examples
///
/// ```
/// let freqs = frequency("ABBA");
/// let probs = freq_to_prob(&freqs);
/// assert_eq!(probs, [('A', 0.5), ('B', 0.5)]);
/// ```
pub fn freq_to_prob(f: &HashMap<char, i32>) -> HashMap<char, f32> {
    let mut d: HashMap<char, f32> = HashMap::new();
    let sum: f32 = f.values().sum::<i32>() as f32;
    for (&symbol, &value) in f.iter() {
        d.insert(symbol, value as f32 / sum);
    }
    d
}


/// Calculates the entropy of a given probability.
///
/// Entropy is the average amount of information contained in each piece of information received
///
/// # Arguments
///
/// * `f` - A reference to a `HashMap` containing character frequencies as `f32`.
///
/// # Returns
///
/// Returns a floating-point number representing the entropy value.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// let mut example1 = HashMap::new();
/// example1.insert('1', 0.25);
/// example1.insert('A', 0.25);
/// example1.insert('B', 0.25);
/// example1.insert('2', 0.125);
/// example1.insert('j', 0.125);
/// assert_eq!(entropy(&example1), 2.25);
///
/// let mut example2 = HashMap::new();
/// example2.insert('A', 0.333);
/// example2.insert('C', 0.166);
/// example2.insert('B', 0.083);
/// example2.insert('D', 0.416);
/// assert_eq!(entropy(&example2), 1.784);
/// ```
///
/// Note that the function signature assumes that you've imported `HashMap` from the standard library.
pub fn entropy(f: &HashMap<char, f32>) -> f32 {
    let mut total: f32 = 0.0;
    for &value in f.values() {
        total += value * -value.log2();
    }
    total
}

/// Calculates the expected length of a message for a given variable length encoding
///
/// The expected length is computed as the sum of the products of the probabilities and lengths of
/// each symbol's encoding.
///
/// # Arguments
///
/// * `f` - A reference to a `HashMap` containing character frequencies as `f32`.
/// * `c` - A reference to a `HashMap` containing values and encoded values as String.
///
/// # Returns
///
/// Returns a floating-point number representing the expected value.
///
///
/// Note that the function signature assumes that you've imported `HashMap` from the standard library.
pub fn expected(f: &HashMap<char, f32>, c: &HashMap<char, &str>) -> Result<f32, SymbolMappingError> {
    let mut total: f32 = 0.0;

    // Check for symbols present in f but not in c
    for (&symbol, &freq) in f.iter() {
        let code = c.get(&symbol).ok_or(SymbolMappingError::SymbolNotFoundInCodes(symbol))?;
        total += freq * (code.len() as f32);
    }

    // Check for symbols present in c but not in f
    for &symbol in c.keys() {
        if !f.contains_key(&symbol) {
            return Err(SymbolMappingError::ExtraSymbolInCodes(symbol));
        }
    }

    Ok(total)
}

pub fn huffman(probs: HashMap<char, f32>) -> HashMap<char, String> {
    let mut heap: BinaryHeap<Node> = probs
        .into_iter()
        .map(|(symbol, probability)| Node {
            probability,
            kind: NodeKind::Leaf { symbol },
        })
        .collect();

    while heap.len() > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        let new_node = Node {
            probability: a.probability + b.probability,
            kind: NodeKind::Internal { left: Box::new(a), right: Box::new(b) },
        };
        heap.push(new_node);
    }

    let root = heap.pop().unwrap();
    let mut huffman_codes = HashMap::new();
    generate_codes(root, String::new(), &mut huffman_codes);

    huffman_codes
}


fn generate_codes(node: Node, current_code: String, huffman_codes: &mut HashMap<char, String>) {
    match node.kind {
        NodeKind::Leaf { symbol } => {
            huffman_codes.insert(symbol, current_code);
        }
        NodeKind::Internal { left, right } => {
            generate_codes(*left, current_code.clone() + "0", huffman_codes);
            generate_codes(*right, current_code + "1", huffman_codes);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[cfg(test)]
    mod expected_tests {
        use super::*;

        #[test]
        fn test_expected() {
            let mut codes: HashMap<char, &str> = HashMap::new();
            codes.insert('A', "11");
            codes.insert('C', "101");
            codes.insert('B', "100");
            codes.insert('D', "0");
            let mut f: HashMap<char, f32> = HashMap::new();
            f.insert('A', 0.333);
            f.insert('B', 0.083);
            f.insert('C', 0.166);
            f.insert('D', 0.416);
            let result = expected(&f, &codes).unwrap();
            assert_relative_eq!(result, 1.8289999, epsilon = 1e-9);
        }

        #[test]
        fn test_mismatch() {
            let mut codes: HashMap<char, &str> = HashMap::new();
            codes.insert('A', "11");
            codes.insert('C', "101");
            codes.insert('B', "100");
            codes.insert('D', "0");
            codes.insert('E', "0000");
            let mut f: HashMap<char, f32> = HashMap::new();
            f.insert('A', 0.333);
            f.insert('B', 0.083);
            f.insert('C', 0.166);
            f.insert('D', 0.416);
            let result = expected(&f, &codes);
            assert!(result.is_err());
            match result {
                Ok(_) => panic!("Expected an Err, got Ok"),
                Err(e) => {
                    match e {
                        SymbolMappingError::ExtraSymbolInCodes(symbol) => {
                            assert_eq!(symbol, 'E');
                        } // Correct error type
                        _ => panic!("Received wrong error type"),
                    }
                },
            }
        }
    }

    #[cfg(test)]
    mod entropy_tests {
        use super::*;

        #[test]
        fn test_entropy() {
            let mut example1 = HashMap::new();
            example1.insert('1', 0.25);
            example1.insert('A', 0.25);
            example1.insert('B', 0.25);
            example1.insert('2', 0.125);
            example1.insert('j', 0.125);
            assert_eq!(entropy(&example1), 2.25);
        }
    }

    #[cfg(test)]
    mod freq_to_prob_tests {
        use super::*;

        #[test]
        fn test_freq_to_prob_abba() {
            let mut input = HashMap::new();
            input.insert('A', 2);
            input.insert('B', 2);

            let result = freq_to_prob(&input);

            assert!(result.contains_key(&'A'));
            assert!(result.contains_key(&'B'));

            assert!((result[&'A'] - 0.5).abs() < f32::EPSILON);
            assert!((result[&'B'] - 0.5).abs() < f32::EPSILON);
        }
    }

    #[cfg(test)]
    mod frequency_tests {
        use super::*;
        #[test]
        fn test_frequency_empty_string() {
            let s = "";
            let expected: HashMap<char, i32> = HashMap::new();
            assert_eq!(frequency(s), expected);
        }

        #[test]
        fn test_frequency_single_character() {
            let s = "A";
            let mut expected = HashMap::new();
            expected.insert('A', 1);
            assert_eq!(frequency(s), expected);
        }

        #[test]
        fn test_frequency_unique_characters() {
            let s = "ABC";
            let mut expected = HashMap::new();
            expected.insert('A', 1);
            expected.insert('B', 1);
            expected.insert('C', 1);
            assert_eq!(frequency(s), expected);
        }

        #[test]
        fn test_frequency_all_same_characters() {
            let s = "AAA";
            let mut expected = HashMap::new();
            expected.insert('A', 3);
            assert_eq!(frequency(s), expected);
        }

        #[test]
        fn test_frequency_mixed_characters() {
            let s = "AABBC";
            let mut expected = HashMap::new();
            expected.insert('A', 2);
            expected.insert('B', 2);
            expected.insert('C', 1);
            assert_eq!(frequency(s), expected);
        }

        #[test]
        fn test_frequency_case_sensitivity() {
            let s = "Aa";
            let mut expected = HashMap::new();
            expected.insert('A', 1);
            expected.insert('a', 1);
            assert_eq!(frequency(s), expected);
        }

        #[test]
        fn test_frequency_non_alphabetic_characters() {
            let s = "A1 !";
            let mut expected = HashMap::new();
            expected.insert('A', 1);
            expected.insert('1', 1);
            expected.insert(' ', 1);
            expected.insert('!', 1);
            assert_eq!(frequency(s), expected);
        }

        #[test]
        fn test_frequency_unicode_characters() {
            let s = "A😀";
            let mut expected = HashMap::new();
            expected.insert('A', 1);
            expected.insert('😀', 1);
            assert_eq!(frequency(s), expected);
        }
    }
}