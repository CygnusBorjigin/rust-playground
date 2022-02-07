use std::env;
use std::fs;

fn main() {
    // take and check user input
    let input_args: Vec<String> = env::args().collect();
    if input_args.len() < 2 {
        eprintln!("please enter the file name");
    } else if input_args.len() > 2 {
        eprintln!("only one argument is accepted");
    };

    // search and open file
    let file_content = fs::read_to_string(&input_args[1]).expect("file cannot be open");

    // parse the file into indivisual words
    let file_char_list = explode(file_content);

    let result = lexer(file_char_list);
    println!("{:?}", result);
}

fn lexer (source: Vec<char>) -> Element {
    return Element::Word(String::from("Working"));
}

fn explode (source: String) -> Vec<char> {
    return source.chars().collect();
}

#[derive(Debug)]
enum Element {
    Word(String),
    Punctuation(char),
    WhiteSpace(i32),
    LineBreak,
}



