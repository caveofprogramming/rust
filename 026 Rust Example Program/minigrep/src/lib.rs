use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query:String = args[1].clone();
        let file_path:String = args[2].clone();
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        Ok(Config{ query, file_path, ignore_case })
    }
}

pub fn run(config: Config)->Result<(), Box<dyn Error>> {
    println!("Searching for '{}' in '{}'", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }
    else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str)->Vec<&'a str> {
    let mut results:Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)->Vec<&'a str> {
    let mut results:Vec<&str> = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}