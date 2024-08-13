mod base;
mod brute_rsa;
mod rot;
mod string_shift;
mod substitution;
mod util;

use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

use base::run_base;
use brute_rsa::brute_force_rsa;
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

        if program == ProgramType::NONE {
            (program, get_input) = get_program_type(&args[i]);
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

    let input_string = match input {
        Some(input) => input,
        _ => panic!("No input provided"),
    };

    match program {
        ProgramType::ROT => run_rot(&input_string),
        ProgramType::Base64Encode => run_base(&input_string, util::CodingType::ENCODE),
        ProgramType::Base64Decode => run_base(&input_string, util::CodingType::DECODE),
        ProgramType::StringShift => string_shift::run(&input_string),
        ProgramType::BruteRsa => brute_force_rsa(&input_string),
        ProgramType::Substitution => substitution::run_substitution(&input_string),
        ProgramType::NONE => println!("No mode selected"),
    }
}

fn get_program_type(arg: &String) -> (ProgramType, bool) {
    return match arg.as_str() {
        "-r" | "--rot" => (ProgramType::ROT, true),
        "-be" | "--base-encode" => (ProgramType::Base64Encode, true),
        "-bd" | "--base-decode" => (ProgramType::Base64Decode, true),
        "--string-shift" => (ProgramType::StringShift, true),
        "--brute-rsa" => (ProgramType::BruteRsa, true),
        "--substitution" | "-s" => (ProgramType::Substitution, true),
        _ => (ProgramType::NONE, false),
    };
}

#[derive(PartialEq)]
enum ProgramType {
    ROT,
    Base64Encode,
    Base64Decode,
    StringShift,
    BruteRsa,
    Substitution,
    NONE,
}
