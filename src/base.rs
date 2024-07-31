use crate::util;
use crate::util::CodingType;
use core::panic;

pub fn run_base(input: &Option<String>, coding_type: CodingType) {
    match coding_type {
        CodingType::ENCODE => {
            if let Some(ref input_string) = input {
                println!("{}", base64_encode(input_string.to_string()));
            } else {
                panic!("No input provided for encoding");
            }
        }
        CodingType::DECODE => {}
    }
}

fn base64_encode(input: String) -> String {
    let mut raw_binary: Vec<String> = vec![];
    for c in input.chars() {
        let code = c as u32;
        let formatted = format!("{code:08b}");
        raw_binary.push(formatted.clone());
    }

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
    assert_eq!("UGFuY2FrZQ==", base64_encode("Pancake".to_string()));
    assert_eq!("SGVsbG8gV29ybGQ=", base64_encode("Hello World".to_string()));
}
