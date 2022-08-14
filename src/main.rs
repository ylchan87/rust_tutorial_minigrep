use std::{env, fs};

fn main() {
    let  args:Vec<String> = env::args().collect();

    let config = Config::new(&args);
    
    

    println!("Search for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Read from file");
    println!("Contents:\n{contents}");

}

struct Config {
    query:String,
    file_path: String,
}

impl Config{
    fn new(args:&Vec<String>)->Self{
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config {query, file_path}       
    }
}