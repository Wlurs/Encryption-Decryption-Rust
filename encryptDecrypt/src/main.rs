use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    println!("Hello, world!");

    let arg: Vec<String> = env::args().collect();
    let data: String = read_file(&arg[1]);
    let shift: u8 = arg[2].parse().expect("Invalid shift value");
    dbg!(&caesar_encrypt(&data, shift));
}

fn read_file(path: &str) -> String
{
    let mut file = File::open(path).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error reading file");
    // println!("File contents:\n{}", data);
    return data;
}

fn caesar_encrypt(data: &String, shift: u8) -> String
{
    let mut encrypted = String::new();
    for c in data.chars(){
        // let shifted_char = ((c as u8 + shift) as char).to_string();
        encrypted.push( (c  as u8 + shift as u8) as char);
    }

    return encrypted;
}

