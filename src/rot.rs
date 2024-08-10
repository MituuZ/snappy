pub fn run_rot(input: &String) {
    for i in 0..26 {
        rot(i, input);
    }
}

pub fn rot(rot_nro: u32, input: &String) -> String {
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
    let alphabet_size = 26;
    let new_code = original_code + rot_nro;
    if original_code >= 65 && original_code <= 65 + alphabet_size - 1 {
        if new_code > 65 + alphabet_size - 1 {
            return new_code - alphabet_size;
        }
        return new_code;
    } else if original_code >= 97 && original_code <= 97 + alphabet_size - 1 {
        if new_code > 97 + alphabet_size - 1 {
            return new_code - alphabet_size;
        }
        return new_code;
    }
    return original_code;
}

#[test]
fn rot_tests() {
    assert_eq!("Hello World", rot(0, &"Hello World".to_string()));
    assert_eq!("Olssv Dvysk", rot(7, &"Hello World".to_string()));
    assert_eq!("Xubbe Mehbt", rot(16, &"Hello World".to_string()));
    assert_eq!("Hello World", rot(26, &"Hello World".to_string()));
}
