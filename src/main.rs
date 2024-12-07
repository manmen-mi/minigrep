use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let path = &args[2];

    println!("Searching {}", query);
    println!("In file {}", path);

    let raw = fs::read_to_string(path)
        .expect("failed: can not read file");

    println!("With text: \n{raw}");
}
