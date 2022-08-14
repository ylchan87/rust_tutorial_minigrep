use std::{env, process};
use minigrep::*;

fn main() {
    let  args:Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(
        |err| {
            eprintln!("Cannot parse args: {err}");
            process::exit(1);
        } 
    );
    
    println!("Search for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config){
        eprintln!("Fail at running, err : {e}");
        process::exit(1);
    }
}