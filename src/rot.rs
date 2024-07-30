pub fn run_rot(input: &Option<String>) {
    if let Some(ref input_string) = input {
        for i in 0..26 {
            rot(i, input_string.to_string());
        }
    } else {
        for i in 0..26 {
            rot(i, "".to_string());
        }
    }
}

fn rot(rot_nro: u32, program_input: String) -> String {
    let input = if program_input.is_empty() {
        "HelloThere"
    } else {
        &program_input
    };
    let mut output_string: Vec<String> = vec![];

    for c in input.chars() {
        let code = c as u32;
        let new_char = char::from_u32(get_next_ascii_code(code, rot_nro));

        match new_char {
            Some(new_char) => {
                output_string.push(new_char.to_string());
            }
            None => {
                println!("Failed to create char from {}", c);
                output_string.push("?".to_string());
            }
        }
    }
    println!("Output with rot {}: {}", rot_nro, output_string.join(""));
    return output_string.join("");
}

fn get_next_ascii_code(original_code: u32, rot_nro: u32) -> u32 {
    let new_code = original_code + rot_nro;
    if original_code >= 65 && original_code <= 90 {
        if new_code > 90 {
            return new_code - 26;
        }
        return new_code;
    } else if original_code >= 97 && original_code <= 122 {
        if new_code > 122 {
            return new_code - 26;
        }
        return new_code;
    }
    return original_code;
}

#[test]
fn rot_13() {
    assert_eq!("Hello World", rot(0, "Hello World".to_string()));
    assert_eq!("Olssv Dvysk", rot(7, "Hello World".to_string()));
    assert_eq!("Xubbe Mehbt", rot(16, "Hello World".to_string()));
    assert_eq!("Hello World", rot(26, "Hello World".to_string()));
}
