use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // let query = &args[1];
    // let file_path = &args[2];

    // old way of defining config
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // new way of defining config (with iterator)
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // let _ = run(config);
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    // println!("With text:\n{contents}");
    
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            query, 
            file_path,
            ignore_case,
        })
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query = &args[1].clone();
//     let file_path = &args[2].clone();

//     Config { query, file_path }
// }
