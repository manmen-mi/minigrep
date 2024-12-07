use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.path);

    let raw = fs::read_to_string(config.path)
        .expect("failed: can not read file");
}


struct Config {
    query: String,
    path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough args");
        }

        let query = args[1].clone();
        let path = args[2].clone();
    
        Config { query, path }
    }
}
