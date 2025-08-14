use std::env;
use std::error::Error;
use std::fs;
use std::process;
use cli::Config;


fn main(){
    let args: Vec<String> = env::args().collect();

    
    let content = Config::new(&args).unwrap_or_else(|err|{
        println!("problem happened while parsing argument {} ",err);
        process::exit(1)
    });
    
    println!("query is {}", content.query);
    
    println!("FILENAME is {}", content.filename);


    if let Err(e)=cli::run(content){
        println!("Application error :{}",e);
        process::exit(1);
    }
}






