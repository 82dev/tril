use std::{env, path::Path, fs::{File, self}};

use lexer::Lexer;

use crate::parser::Parser;

mod token;
mod lexer;
mod parser;
mod node;
mod r#type;

fn main() {
  let args: Vec<String> = env::args().collect();
  let path = Path::new(&args[1]);
  let src = fs::read_to_string(path)
    .expect("Couldn't read specified file!");

  println!("{}", src);
  
  let lexer = Lexer::new(src);

  // lexer.lex().into_iter()
  //   .for_each(|a|{
  //     println!("{:?},", a);
  //   });

  let tokens = lexer.lex();

  tokens.iter()
    .for_each(|a|{
      println!("{:?},", a);
  });

  let parser = Parser::new(tokens);
  parser.parse().into_iter()
    .for_each(|a|{
      println!("{:?}", a)
    })
}
