use crate::util::CodingType;
use crate::util::{self, substring_with_padding};
use core::panic;

pub fn run_base(input: &Option<String>, coding_type: CodingType) {
    match coding_type {
        CodingType::ENCODE => {
            if let Some(ref input_string) = input {
                println!("{}", base64_encode(input_string));
            } else {
                panic!("No input provided for encoding");
            }
        }
        CodingType::DECODE => {
            if let Some(ref input_string) = input {
                println!("{}", base64_decode(input_string));
            } else {
                panic!("No input provided for decoding");
            }
        }
    }
}

fn base64_decode(input: &str) -> String {
    let mut owned_string: String = input.to_string();
    while owned_string.chars().last() == Some('=') {
        owned_string = substring_with_padding(&owned_string, 0, owned_string.len() - 1);
    }
    println!("Padding removed from string: {}", owned_string);

    let raw_binary: Vec<String> = get_binary_string(&owned_string, 6);
    println!("Raw binary string: {}", raw_binary.join(""));

    return "".to_string();
}

fn base64_encode(input: &str) -> String {
    let mut raw_binary: Vec<String> = get_binary_string(input, 8);

    let mut i = 0;
    while raw_binary.join("").len() % 6 != 0 {
        raw_binary.push('0'.to_string());
        if i == 25 {
            panic!("Misaligned binary data");
        }
        i += 1;
    }

    let joined_binary = raw_binary.join("");
    let mut chunks: Vec<String> = vec![];
    let mut start = 0;
    let chunk_size = 6;

    while start < joined_binary.len() {
        let chunk = util::substring_with_padding(&joined_binary, start, start + chunk_size);
        chunks.push(chunk);
        start += chunk_size;
    }
    return process_chunks(&chunks);
}

fn get_binary_string(input: &str, size: usize) -> Vec<String> {
    let mut raw_binary: Vec<String> = vec![];

    for c in input.chars() {
        let code = c as u32;
        let formatted = match size {
            8 => format!("{code:08b}"),
            6 => format!("{code:06b}"),
            _ => panic!("Unrecognized binary formatter"),
        };
        raw_binary.push(formatted.clone());
    }
    return raw_binary;
}

fn process_chunks(chunks: &Vec<String>) -> String {
    let mut result: Vec<String> = vec![];
    for chunk in chunks {
        result.push(get_ascii(&chunk));
    }
    while result.join("").len() % 4 != 0 {
        result.push("=".to_string());
    }
    return result.join("");
}

fn get_ascii(binary: &str) -> String {
    const BASE64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    match u8::from_str_radix(binary, 2) {
        Ok(num) => match BASE64_CHARS.chars().nth(num as usize) {
            Some(val) => val.to_string(),
            None => String::new(),
        },
        Err(_) => {
            panic!("Failed to convert binary to ASCII");
        }
    }
}

#[test]
fn base_tests() {
    assert_eq!("UGFuY2FrZQ==", base64_encode("Pancake"));
    assert_eq!("SGVsbG8gV29ybGQ=", base64_encode("Hello World"));
}
