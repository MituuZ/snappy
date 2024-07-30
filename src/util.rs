/// Get the substring from a &String, also pads the string if the length is too short
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

#[test]
fn base_substring() {
    assert_eq!("asd", substring_with_padding(&"asdf".to_string(), 0, 3));
    assert_eq!("asd", substring_with_padding(&"asd".to_string(), 0, 3));
    assert_eq!("as", substring_with_padding(&"asd".to_string(), 0, 2));
    assert_eq!("asd00", substring_with_padding(&"asd".to_string(), 0, 5));
    assert_eq!("", substring_with_padding(&"asd".to_string(), 0, 0));
}
