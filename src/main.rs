use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = "None";

    let mut i = 0;
    while i < args.len() {
        i += 1;
    }

    if program == "rot" {
        rot();
    } else {
        println!("No valid program selected");
    }
}

fn rot() {
    let input = "HelloThere";
    let mut output_string: Vec<String> = vec![];

    for c in input.chars() {
        let code = c as u32;
        let new_char = char::from_u32(get_next_ascii_code(code));

        match new_char {
            Some(new_char) => {
                println!("{} as dec: {} + 13: {}", c, code, new_char);
                output_string.push(new_char.to_string());
            }
            None => println!("Failed to create char from {}", c),
        }
    }
    println!("");
    println!("Output: {}", output_string.join(""));
}

fn get_next_ascii_code(original_code: u32) -> u32 {
    let new_code = original_code + 13;
    if original_code > 64 && original_code < 91 {
        if new_code > 90 {
            return new_code - 26;
        }
    } else {
        if new_code > 122 {
            return new_code - 26;
        }
    }
    return new_code;
}
