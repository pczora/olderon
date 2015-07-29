use machine_state::OpCode;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use regex::Regex;
use std::num;

// TODO: Well, actually parse a given file into a Vector<OpCode> which is than to be run by the VM,
// you doofus
pub fn parse(path_string: &String) -> Vec<OpCode> {
    let mut program: Vec<OpCode> = Vec::new();
    let path = Path::new(path_string);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(..)  => panic!("room"),
    };
    let mut reader = BufReader::new(&file);
    let s = &mut String::new();
    for line in reader.lines() {
        //match line {
            //Ok(x) => parse_line(x),
            ////Ok(x) => println!("OK!: {}", x),
            //Err(x) => println!("Err: {}", x),
        //}
        program.push(parse_line(line.unwrap()));
    }
    program
}

pub fn parse_line(line: String) -> OpCode {
    let psh_regex = Regex::new(r"^PSH (\d*)").unwrap();
    let pop_regex = Regex::new(r"^POP").unwrap();
    let add_regex = Regex::new(r"^ADD").unwrap();
    let sub_regex = Regex::new(r"^SUB").unwrap();
    let mul_regex = Regex::new(r"^MUL").unwrap();
    let div_regex = Regex::new(r"^DIV").unwrap();
    let jmp_regex = Regex::new(r"^JMP (\d*)").unwrap();
    let jmpeq_regex = Regex::new(r"^JMPEQ (\d*)").unwrap();
    let hlt_regex = Regex::new(r"^HLT").unwrap();

    let opcode: OpCode = if psh_regex.is_match(&line) {
        let cap = psh_regex.captures(&line).unwrap();
        let arg = cap.at(1).unwrap();
        OpCode::PSH(arg.parse().unwrap())
    }
    else if pop_regex.is_match(&line) {
        OpCode::POP
    }
    else if add_regex.is_match(&line) {
        OpCode::ADD
    }
    else if sub_regex.is_match(&line) {
        OpCode::SUB
    }
    else if mul_regex.is_match(&line) {
        OpCode::MUL
    }
    else if div_regex.is_match(&line) {
        OpCode::DIV
    }
    else if jmp_regex.is_match(&line) {
        let cap = jmp_regex.captures(&line).unwrap();
        let arg = cap.at(1).unwrap();
        OpCode::JMP(arg.parse().unwrap())
    }
    else if jmpeq_regex.is_match(&line) {
        let cap = jmpeq_regex.captures(&line).unwrap();
        let arg = cap.at(1).unwrap();
        OpCode::JMPEQ(arg.parse().unwrap())
    }
    else {
        OpCode::HLT
    };

    return opcode;
}
