mod token;
mod lexer;

use std::{env, fs};

use crate::{token::Token, lexer::Lexer};

fn main() {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];

  let contents = fs::read_to_string(file_path)
                      .expect(&format!("Couldn't read file: [{file_path}]"));
  println!("{}", contents);

  let mut lexer = Lexer::new(contents);
  println!("{:?}", lexer.tokenize());
}
