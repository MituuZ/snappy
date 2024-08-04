use crate::util::CodingType;
use crate::util::{self, substring_with_padding};
use core::panic;
use std::char;

static BASE64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

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
    let raw_binary: Vec<String> = get_binary_from_ascii(&owned_string);

    let mut result: Vec<String> = vec![];
    let chunks: Vec<String> = create_chunks(&raw_binary.join(""), 8);
    for chunk in chunks {
        if chunk.len() != 8 {
            continue;
        }
        let asd = match u8::from_str_radix(&chunk, 2) {
            Ok(val) => val,
            _ => panic!("Failed to convert binary to a number"),
        };
        let new_char = match char::from_u32(asd as u32) {
            Some(val) => val,
            _ => panic!("Failed to convert binary to a number"),
        };
        if chunk != "00000000" {
            result.push(new_char.to_string());
        };
    }

    return result.join("");
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

    let chunks: Vec<String> = create_chunks(&raw_binary.join(""), 6);
    return process_chunks(&chunks);
}

fn create_chunks(binary_string: &String, size: usize) -> Vec<String> {
    let mut chunks: Vec<String> = vec![];
    let mut start = 0;

    while start < binary_string.len() {
        let chunk = util::substring_with_padding(&binary_string, start, start + size);
        chunks.push(chunk);
        start += size;
    }
    return chunks;
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

fn get_binary_from_ascii(input: &str) -> Vec<String> {
    let mut raw_binary: Vec<String> = vec![];

    for ascii_char in input.chars() {
        match BASE64_CHARS.find(ascii_char) {
            Some(val) => raw_binary.push(format!("{val:06b}")),
            _ => panic!("No ascii rep found for char"),
        };
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
fn base64_encode_tests() {
    assert_eq!("UGFuY2FrZQ==", base64_encode("Pancake"));
    assert_eq!("SGVsbG8gV29ybGQ=", base64_encode("Hello World"));
    assert_eq!(
        "U29tZWJvZHkgb25jZSB0b2xkIG1lLCB0aGUgd29ybGQgaXMgZ29ubmEgcm9sbCBtZQ==",
        base64_encode("Somebody once told me, the world is gonna roll me")
    );
}

#[test]
fn base64_decode_tests() {
    assert_eq!("Pancake", base64_decode("UGFuY2FrZQ=="));
    assert_eq!("Hello World", base64_decode("SGVsbG8gV29ybGQ="));
    assert_eq!(
        "Somebody once told me, the world is gonna roll me",
        base64_decode("U29tZWJvZHkgb25jZSB0b2xkIG1lLCB0aGUgd29ybGQgaXMgZ29ubmEgcm9sbCBtZQ==")
    );
}
