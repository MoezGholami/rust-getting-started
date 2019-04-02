use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;

const TEMP_FILE_NAME : &str = "temp.txt";
const TEMP_FILE_CONTENT : &str = "some content\nanother content\n";

pub fn main() {
    //basic file io without exception handling
    println!("current directory is: {}",
             env::current_dir().expect("could not load the current directory.").display());
    write_demo();
    read_demo();
    clean_up();
    stdin_demo();
}

fn write_demo() {
    let mut write : File = File::create(TEMP_FILE_NAME).expect("could not create file");
    write.write_all(TEMP_FILE_CONTENT.as_bytes()).expect("could not write in file");
}

fn read_demo() {
    let mut read : File = File::open(TEMP_FILE_NAME).expect("could not open the file");
    let mut content : String = String::new();
    read.read_to_string(&mut content).expect("could not read the file");
    println!("here is the content:\n{}", content);
}

fn clean_up() {
    fs::remove_file(TEMP_FILE_NAME).expect("could not remove file");
}

fn stdin_demo() {
    let mut name : String = String::new();
    println!("what is your name dear user? ...");
    stdin().read_line(&mut name).expect("could not read your name :/");
    println!("Hello {}!", name.trim());
}
