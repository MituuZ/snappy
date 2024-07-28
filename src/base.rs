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
}
