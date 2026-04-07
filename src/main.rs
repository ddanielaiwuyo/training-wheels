use std::env;
use std::fs;
use std::process;

use minigrep::search;
use minigrep::search_case_insensitive;

#[derive(Debug)]
struct Config {
    query: String,
    src_file: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let src_file = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            src_file,
            ignore_case,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    println!("Query: {}", config.query);
    run(&config)
}

fn run(config: &Config) {
    let contents = fs::read_to_string(&config.src_file).unwrap_or_else(|err| {
        eprintln!("Could not get {}. {err}", config.src_file);
        process::exit(1);
    });

    let result = if config.ignore_case {
        search_case_insensitive(&config.query.as_str(), contents.as_str())
    } else {
        search(&config.query.as_str(), contents.as_str())
    };

    for line in result {
        println!("{line}")
    }
}
