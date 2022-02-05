use std::fs;
use std::process; // for exit the program without panic
use std::error::Error; // for construction of the error type

pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new( args: &[String] ) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("there isn't enough argument...");
        }
         let query = args[1].clone();
         let file_name = args[2].clone();

         return Ok(Config{ query, file_name });
    }
}

pub fn run ( config: Config ) -> Result<(), Box<dyn Error>>{
     let content = fs::read_to_string(&config.file_name)?;
     println!("The query is: {}", &config.query);
     println!("The file name is: {}", &config.file_name);
     println!("{}", &content);
     return Ok(());
}
