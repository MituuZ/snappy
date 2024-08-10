use std::char;

use crate::util::substring;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn run(input: &String) {
    for i in 0..25 {
        run_shifts(input, i);
    }
}

fn run_shifts(input: &String, shift: usize) {
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

        //let new_char = match char::from_digit(char_index as u32, 16) {
        let new_char = match ALPHABET.chars().nth(char_index as usize % ALPHABET.len()) {
            Some(val) => val,
            None => '*',
        };
        res.push(new_char.to_string());
        println!("chars: {two_chars}, bin: {bin_rep}, index: {char_index}, char: {new_char}");
        i += 2;
    }

    println!("{}", res.join(""));
}

fn morph_to_binary(input: &str) -> String {
    let mut raw_binary: Vec<String> = vec![];

    for c in input.chars() {
        match ALPHABET.find(c) {
            Some(val) => raw_binary.push(format!("{val:04b}")),
            _ => raw_binary.push("0000".to_string()),
        };
    }

    return raw_binary.join("");
}

fn get_binary_string(input: &str) -> Vec<String> {
    let mut raw_binary: Vec<String> = vec![];

    for c in input.chars() {
        let code = c as u32;
        println!("{c}: {code}: {:04b}", code & 0b1111);
        raw_binary.push(format!("{:04b}", code & 0b1111));
    }
    return raw_binary;
}
