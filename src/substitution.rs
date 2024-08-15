use core::panic;

static ABC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
static SUB: &str = "gmlzjoxwtnisruvqpebkhfdcyaGMLZJOXWTNISRUVQAEBKHFDCYP";

pub fn run_substitution(input: &String) {
    let mut res: Vec<String> = vec![];
    for c in input.chars() {
        let index = match SUB.find(c) {
            Some(val) => val,
            _ => {
                res.push(c.to_string());
                continue;
            }
        };

        let new_char = match ABC.chars().nth(index) {
            Some(val) => val,
            _ => panic!("No new char found"),
        };
        res.push(new_char.to_string());
    }
    println!("{input}");
    println!("\nResult:\n{}", res.join(""));

    get_frequencies(input)
}

fn get_frequencies(input: &String) {
    println!("\nCipher frequencies: ");

    let mut input_freq: Vec<(char, usize)> = vec![];
    for c in input.chars() {
        increment_or_insert(&mut input_freq, c)
    }

    input_freq.sort_by(|a, b| b.1.cmp(&a.1));
    let length = input.len();

    for (key, val) in input_freq {
        let freq: f32 = val as f32 / length as f32;
        if ABC.contains(key) {
            println!("{key}: {freq}");
        }
    }
}

fn increment_or_insert(input_freq: &mut Vec<(char, usize)>, c: char) {
    let lower = c.to_ascii_lowercase();
    for v in input_freq.iter_mut() {
        if v.0 == lower {
            v.1 += 1;
            return;
        }
    }
    input_freq.push((lower, 1));
}
