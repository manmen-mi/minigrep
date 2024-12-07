use std::{error::Error, fs};

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let path = args[2].clone();
    
        Ok(Config { query, path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string(config.path)?;
    for line in search(&config.query, &raw) {
        println!("{line}");
    }

    Ok(())
}
