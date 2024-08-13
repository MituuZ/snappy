use core::panic;

static ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static SUB: &str = "DECKFMYIQJRWTZPXGNABUSOLVH";

pub fn run_substitution(input: &String) {
    let mut res: Vec<String> = vec![];
    for c in input.chars() {
        let uppercase = c.to_uppercase().to_string();
        let index = match SUB.find(&uppercase) {
            Some(val) => val,
            _ => {
                res.push(uppercase);
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
    println!("{}", res.join(""));
}
