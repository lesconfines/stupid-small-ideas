use std::env;
use std::fs;
use std::process::exit;

mod json_interpreter;

use json_interpreter::parser::Parser;
use json_interpreter::token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("json_parser [json file]");
        exit(1);
    }
    let input = fs::read_to_string(args[1].to_string()).expect("Error: Filepath is not correct");
    match Parser::new(&input).parse() {
        Ok(json_value) => println!("{}", json_value),
        Err(err) => println!("{}", err),
    }
}
