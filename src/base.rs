use core::panic;

pub fn run_base(input: &Option<String>) {
    if let Some(ref input_string) = input {
        base(input_string.to_string());
    } else {
        base("".to_string());
    }
}

fn base(input: String) {
    let mut raw_binary: Vec<String> = vec![];
    for c in input.chars() {
        let code = c as u32;
        let formatted = format!("{code:b}");
        raw_binary.push(formatted.clone());
    }
    println!("source: {}", input);
    println!("target: {}", raw_binary.join(""));

    let mut i = 0;
    while raw_binary.len() % 3 != 0 {
        let code = '=' as u32;
        let filler = format!("{code:b}");
        raw_binary.push(filler.clone());
        if i == 5 {
            panic!("Misaligned binary data");
        }
        i += 1;
    }
    println!("padded target: {}", raw_binary.join(""));

    let joined_binary = raw_binary.join("");
    let mut chunks: Vec<String> = vec![];
    let mut start = 0;
    let chunk_size = 6;
    let chars: Vec<char> = joined_binary.chars().collect();

    while start < joined_binary.len() {
        let chunk = substring(&chars, start, start + chunk_size);
        chunks.push(chunk);
        start += chunk_size;
    }

    for chunk in chunks {
        println!("Chunk: {}", chunk);
    }
}

fn substring(chars: &Vec<char>, start: usize, end: usize) -> String {
    let mut result: Vec<char> = vec![];
    for i in start..end {
        result.push(chars[i]);
    }
    // Dude, what the hell is this
    return result.iter().cloned().collect::<String>();
}
