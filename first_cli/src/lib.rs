use std::fs;
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

     for line in search(&config.query, &content) {
        println!("{}", line);
     }

     return Ok(());
}

pub fn search <'a> (query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*; // import everything from the parent
    #[test]
    fn one_result () {
        let query = "content";
        let content = "content of the first test";

        assert_eq!(vec!["content of the first test"], search(query, content));
    }
}