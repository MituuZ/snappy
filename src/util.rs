/// Get the substring from a &String, also pads the string with zeroes, if the length is too short
/// Start is inclusive
/// End is exclusive
pub fn substring_with_padding(source: &String, start: usize, end: usize) -> String {
    let chars: Vec<char> = source.chars().collect();
    let mut result: Vec<char> = vec![];
    for i in start..end {
        if i >= chars.len() {
            result.push('0');
        } else {
            result.push(chars[i]);
        }
    }
    // Dude, what the hell is this
    return result.iter().cloned().collect::<String>();
}

/// Get the substring from a &String
/// Start is inclusive
/// End is exclusive
pub fn substring(source: &String, start: usize, end: usize) -> String {
    let chars: Vec<char> = source.chars().collect();
    let mut result: Vec<char> = vec![];
    let mut i = start;
    while i < chars.len() && i < end {
        result.push(chars[i]);
        i += 1;
    }
    return result.iter().cloned().collect::<String>();
}

pub enum CodingType {
    ENCODE,
    DECODE,
}

#[test]
fn substring_tests() {
    assert_eq!("asd", substring(&"asdf".to_string(), 0, 3));
    assert_eq!("asd", substring(&"asd".to_string(), 0, 3));
    assert_eq!("as", substring(&"asd".to_string(), 0, 2));
    assert_eq!("asd", substring(&"asd".to_string(), 0, 5));
    assert_eq!("", substring(&"asd".to_string(), 0, 0));
}

#[test]
fn substring_with_padding_tests() {
    assert_eq!("asd", substring_with_padding(&"asdf".to_string(), 0, 3));
    assert_eq!("asd", substring_with_padding(&"asd".to_string(), 0, 3));
    assert_eq!("as", substring_with_padding(&"asd".to_string(), 0, 2));
    assert_eq!("asd00", substring_with_padding(&"asd".to_string(), 0, 5));
    assert_eq!("", substring_with_padding(&"asd".to_string(), 0, 0));
}
