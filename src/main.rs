mod rot;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut program = "None";
    let mut input: Option<String> = None;

    let mut i = 0;
    let mut get_input = false;
    while i < args.len() {
        if get_input {
            if args[i] == "-f" || args[i] == "--file" {
                if let Some(file_path) = args.get(i + 1) {
                    match fs::read_to_string(file_path) {
                        Ok(content) => input = Some(content),
                        Err(err) => panic!("File reading failed: {}", err),
                    }
                    i += 1;
                } else {
                    println!("No file provided!");
                }
            } else {
                input = Some(args[i].clone());
            }
        }
        if args[i] == "-r" || args[i] == "--rot" {
            program = "rot";
            get_input = true;
        }
        i += 1;
    }

    if program == "rot" {
        if let Some(ref input_string) = input {
            for i in 0..26 {
                rot::rot(i, input_string.to_string());
            }
        } else {
            for i in 0..26 {
                rot::rot(i, "".to_string());
            }
        }
    }
}
