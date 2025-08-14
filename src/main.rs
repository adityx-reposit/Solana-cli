use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();

    
    let content = Config::new(&args).unwrap();
    
    println!("query is {}", content.query);

    println!("FILENAME is {}", content.filename);
}

struct Config{
    query:String,
    filename:String
}
impl Config {
    
    fn new(args:&[String]) -> Result<Config,&str> {
        if args.len() < 3 {
            eprintln!("Usage: {} <query> <filename>", args[0]);
            std::process::exit(1);
        }
        let query = args[1].clone();
        let filename=args[2].clone();
    
        Ok(Config{ query, filename })
    }
}




