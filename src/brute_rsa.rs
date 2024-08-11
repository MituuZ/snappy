use core::panic;

pub fn brute_force_rsa(input: &String) {
    let cipher: String;
    let n: String;
    let e: String;

    (cipher, n, e) = parse_file_contents(input);

    println!("cipher: {cipher}");
    println!("n: {n}");
    println!("e: {e}");
}

fn parse_file_contents(input: &String) -> (String, String, String) {
    let mut cipher: String = "".to_string();
    let mut n: String = "".to_string();
    let mut e: String = "".to_string();

    for line in input.split("\n") {
        if line.starts_with("c: ") {
            let split = line.split("c: ").collect::<Vec<&str>>();
            cipher = split[1].to_string();
        }
        if line.starts_with("n: ") {
            let split = line.split("n: ").collect::<Vec<&str>>();
            n = split[1].to_string();
        }
        if line.starts_with("e: ") {
            let split = line.split("e: ").collect::<Vec<&str>>();
            e = split[1].to_string();
        }
    }

    if cipher.is_empty() {
        print_file_format();
        panic!("No cipher provided");
    }
    if n.is_empty() {
        print_file_format();
        panic!("No n provided");
    }
    if e.is_empty() {
        print_file_format();
        panic!("No e provided");
    }

    return (cipher, n, e);
}

fn print_file_format() {
    println!(
        "You should provide a file or a string that contains the cipher, n and e.
Each on their own line and the lines should start with the following values:
c:
n:
e:"
    );
}
