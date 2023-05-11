mod token;
mod lexer;
mod parser;
mod nodes;
mod semantic;

use std::{env, fs};

use crate::{lexer::Lexer, parser::Parser};

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2{
    println!("{:?}", args);
    todo!("Print usage");
  }
  
  let file_path = &args[1];

  let contents = fs::read_to_string(file_path)
                      .expect(&format!("Couldn't read file: [{file_path}]"));
  println!("{}", contents);

  let tok = Lexer::new(contents).tokenize();
  println!("{:?}\n\n", tok);
  println!("{:?}", Parser::new(tok).parse());
}
