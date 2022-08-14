use std::fs;
use std::error::Error;

pub struct Config {
    pub query:String,
    pub file_path: String,
}

impl Config{
    pub fn build(args:&Vec<String>)->Result<Self, &'static str>{
        if args.len()<3{
            return Err("Not enough args");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        return Ok(Config {query, file_path});
    }
}

pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    println!("Contents:\n{contents}");
    return Ok(());
}