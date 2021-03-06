use std::fs;
use std::error::Error; // for construction of the error type

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new( args: &[String] ) -> Result<Config, &str> {

        if args.len() < 4 {
            return Err("there isn't enough argument...");
        }
         let query = args[1].clone();
         let file_name = args[2].clone();
         let case_sensitive = if args[3] == "true" { true } else { false };

        return Ok(Config{ query, file_name, case_sensitive });
    }
}

pub fn run ( config: Config ) -> Result<(), Box<dyn Error>>{
     let content = fs::read_to_string(&config.file_name)?;

     let result = if config.case_sensitive {
         search(&config.query, &content)
     } else {
         search_case_insensitive(&config.query, &content)
     };

     println!("{:?}", result);

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

pub fn search_case_insensitive <'a> (query: &str, content: &'a str) -> Vec<&'a str> {
    let query_internal = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query_internal) {
            result.push(line);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*; // import everything from the parent
    #[test]
    fn case_sensitive () {
        let query = "content";
        let content = "content of the first test";

        assert_eq!(vec!["content of the first test"], search(query, content));
    }

    #[test]
    fn case_insensitive () {
        let query = "SometHinG";
        let content = "someone somewhere doing something";

        assert_eq!(vec!["someone somewhere doing something"], search_case_insensitive(query, content));
    }
}

