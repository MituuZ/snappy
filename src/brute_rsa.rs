use core::panic;
use num::bigint::BigUint;

pub fn brute_force_rsa(input: &String) {
    println!("Not implemented! In progess");

    let cipher: BigUint;
    let n: BigUint;
    let e: BigUint;

    (cipher, n, e) = parse_file_contents(input);

    println!("cipher: {cipher}");
    println!("n: {n}");
    println!("e: {e}");

    run_decrypt(cipher, n, e);
}

fn run_decrypt(cipher: BigUint, n: BigUint, e: BigUint) {
    if cipher < n {
        println!("Running small e attack, because c < n");

        let m = cipher.cbrt();
        println!("{m}");

        let bytes = m.to_bytes_be();

        let messsage = match String::from_utf8(bytes) {
            Ok(val) => val,
            _ => "Couldn't transform utf8 to String".to_string(),
        };
        println!("{messsage}");
    } else {
        println!("Not implemented");
    }
}

fn parse_file_contents(input: &String) -> (BigUint, BigUint, BigUint) {
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

    return (
        cipher.parse().unwrap(),
        n.parse().unwrap(),
        e.parse().unwrap(),
    );
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
