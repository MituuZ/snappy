use core::panic;

// SUB maps to the ABC table
// e.g. a character from the original message turns into the one below it
static SUB: &str = "xhkyjlenqbowugmvidsfapcztrXHKYJLENQBOWUGMVIDSFAPCZTR";
static ABC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

// Add chars here that are confirmed
static CONFIRMED: &str = "picotfnem";

// A list of characters in the order of most common first
static FREQ: &str = "etaoinshrdlcumwfgypbvkjxqz";

pub fn run_substitution(input: &String) {
    get_frequencies(input);

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
    println!("Input:\n{input}");
    println!("\nResult:\n{}", res.join(""));

    validate_sub();
}

/// Checks if SUB contains duplicate characters and if all chars from ABC are present in SUB
fn validate_sub() {
    let mut existing: Vec<char> = vec![];
    for c in SUB.chars() {
        if existing.contains(&c) {
            println!("Found a duplicate char: {c} in sub");
        } else {
            existing.push(c);
        }
    }

    for c in ABC.chars() {
        if !existing.contains(&c) {
            println!("Missing char: {c} from sub");
        }
    }
}

/// Print out cipher character frequencies, suggestions based on the FREQ string, current selection
/// and if a character pair has been confirmed
fn get_frequencies(input: &String) {
    println!("\nCipher frequencies\tPossible char\tCurrent char\tConfirmed");

    let mut input_freq: Vec<(char, usize)> = vec![];
    for c in input.chars() {
        increment_or_insert(&mut input_freq, c)
    }

    input_freq.sort_by(|a, b| b.1.cmp(&a.1));
    let length = input.len();

    let mut i = 0;
    for (key, val) in input_freq {
        let freq: f32 = val as f32 / length as f32;
        let freq_char = match FREQ.chars().nth(i) {
            Some(val) => val,
            _ => '*',
        };
        if ABC.contains(key) {
            let current_match = get_current_match(&key);
            let confirmed = if CONFIRMED.contains(current_match) {
                "x"
            } else {
                ""
            };
            println!("{key}: {freq}\t\t{freq_char}\t\t{current_match}\t\t{confirmed}");
            i += 1;
        }
    }
}

/// Find character's index from SUB and return corresponding char from ABC
fn get_current_match(c: &char) -> char {
    let index = match SUB.find(*c) {
        Some(val) => val,
        _ => panic!("Couldn't find match for char {c}"),
    };

    return match ABC.chars().nth(index) {
        Some(val) => val,
        _ => panic!("Couldn't find match for char {c}"),
    };
}

/// Either increment character count or add the character to input_freq vector
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
