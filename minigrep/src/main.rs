use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem encounterd while running: {}", err);
        process::exit(1);
    });

    println!("Searching for : {}", config.query);
    println!("File : {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong when opening the file.");

    println!("With the text :\n\n{}\n", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("too few args.  expecting arguments of: <query> <filename>");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
