// use std::env;
use std::fs::File;
use std::io::prelude::*;
// use std::process::Output;
// use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    shift: u8,
}

fn main() {
    
    let cli = Cli::parse();
    let data: String = read_file(&cli.path as &str);
    let shift: u8 = cli.shift;

    dbg!(&caesar_encrypt(&data, shift));
}

fn read_file(path: &str) -> String
{
    let mut file = File::open(path).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error reading file");
    return data;
}

fn caesar_encrypt(data: &String, shift: u8) -> String
{
    let mut encrypted = String::new();
    for c in data.chars(){
        encrypted.push( (c  as u8 + shift as u8) as char);
    }
    return encrypted;
}

