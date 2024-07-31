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
    let mut program: ProgramType = ProgramType::NONE;
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
            program = ProgramType::ROT;
            get_input = true;
        }
        if args[i] == "-be" || args[i] == "--base-encode" {
            program = ProgramType::Base64Encode;
            get_input = true;
        }
        if args[i] == "-bd" || args[i] == "--base-decode" {
            program = ProgramType::Base64Decode;
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

    match program {
        ProgramType::ROT => run_rot(&input),
        ProgramType::Base64Encode => run_base(&input, util::CodingType::ENCODE),
        ProgramType::Base64Decode => run_base(&input, util::CodingType::DECODE),
        ProgramType::NONE => println!("No mode selected"),
    }
}

enum ProgramType {
    ROT,
    Base64Encode,
    Base64Decode,
    NONE,
}
