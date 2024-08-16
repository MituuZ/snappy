use std::char;

use crate::util::substring;

static ALPHABET: &str = "abcdefghijklmnop";

/// A mix of base16 and rotation encoding. Solves a single picoCTF challenge
pub fn run(input: &String) {
    for i in 0..ALPHABET.len() {
        let input_string = shift_input(input, i);
        println!("\nRunning shift with input: {input_string}");
        run_shift(&input_string);
    }
}

fn shift_input(input: &String, shift: usize) -> String {
    let mut res: Vec<String> = vec![];

    for c in input.chars() {
        let new_index = match ALPHABET.find(c) {
            Some(val) => (val + shift) % ALPHABET.len(),
            _ => panic!("Failed to find char {c} in alphabet"),
        };

        match ALPHABET.chars().nth(new_index) {
            Some(val) => res.push(val.to_string()),
            _ => panic!("Failed to find char in alphabet with index {new_index}"),
        }
    }

    return res.join("");
}

fn run_shift(input: &String) {
    let mut res: Vec<String> = vec![];
    let mut i = 0;
    let mut two_chars = substring(input, i, 2);

    while !two_chars.is_empty() {
        two_chars = substring(input, i, i + 2);
        let bin_rep = morph_to_binary(&two_chars);
        let char_index = match u8::from_str_radix(&bin_rep, 2) {
            Ok(num) => num,
            _ => 0,
        };

        let new_char = match char::from_u32(char_index as u32) {
            Some(val) => val,
            _ => panic!("Failed to turn index {char_index} to char"),
        };
        res.push(new_char.to_string());
        i += 2;
    }

    println!("{}", res.join(""));
}

fn morph_to_binary(input: &str) -> String {
    let mut raw_binary: Vec<String> = vec![];

    for c in input.chars() {
        match ALPHABET.find(c) {
            Some(val) => raw_binary.push(format!("{val:04b}")),
            _ => panic!("Failed to morph value {c} to binary"),
        };
    }

    return raw_binary.join("");
}
