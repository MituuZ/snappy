mod base;
mod rot;
mod util;

use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

use base::run_base;
use rot::run_rot;

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
        if args[i] == "-be" || args[i] == "--base-encode" {
            program = "base-encode";
            get_input = true;
        }
        if args[i] == "-bd" || args[i] == "--base-decode" {
            program = "base-decode";
            get_input = true;
        }
        i += 1;
    }

    if input.is_none() {
        let stdin = io::stdin();
        let mut res: Vec<String> = vec![];
        for line in stdin.lock().lines() {
            res.push(line.unwrap());
        }
        input = Some(res.join(""));
    }

    if program == "rot" {
        run_rot(&input);
    } else if program == "base-encode" {
        run_base(&input, util::CodingType::ENCODE);
    } else if program == "base-decode" {
        run_base(&input, util::CodingType::DECODE);
    }
}
