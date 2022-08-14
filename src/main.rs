use std::{env, fs};

fn main() {
    let  args:Vec<String> = env::args().collect();

    let config = parse_config(&args);
    
    

    println!("Search for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Read from file");
    println!("Contents:\n{contents}");

}

struct Config {
    query:String,
    file_path: String,
}

fn parse_config(args:&Vec<String>) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config {query, file_path}
}