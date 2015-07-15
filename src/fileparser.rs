use machine_state::OpCode;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

// TODO: Well, actually parse a given file into a Vector<OpCode> which is than to be run by the VM,
// you doofus
pub fn parse(path_string: &String) {
   let path = Path::new(path_string);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(..)  => panic!("room"),
    };

    let mut s = String::new();
    file.read_to_string(&mut s);
    println!("{}", s);
}
