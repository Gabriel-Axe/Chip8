use std::{env, fs};

fn main() {
    let data = fs::read("src/hi.md").unwrap();
    println!("{}", String::from_utf8(data).unwrap());
    // for smt in data {
        // println!("{}", String::from_utf8(smt);
    // }
    // let cur_dir = env::current_dir().unwrap();
    // println!("{}", cur_dir.display());
    // let data: Vec<u8> = fs::read(path)
}
