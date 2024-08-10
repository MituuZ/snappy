use crate::util::{self, substring_with_padding};

pub fn run(input: &String) {
    let mut two_chars = substring_with_padding(input, 0, 2);
    while two_chars != "  " {}
}
