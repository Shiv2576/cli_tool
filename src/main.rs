use std::env;
use std::process;

use cli_app::Config;



fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing {}" , err);
        process::exit(1);   
    });
    
    println!("searching for: {}", config.query);
    println!("In file {}" , config.filename);

    if let Err(e) = cli_app::run(config){
        println!("Application Error : {}", e);
        process::exit(1);
    }
}

