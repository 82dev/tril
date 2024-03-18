use std::{env, path::Path, fs::{File, self}};

use lexer::Lexer;

mod token;
mod lexer;

fn main() {
  let args: Vec<String> = env::args().collect();
  let path = Path::new(&args[1]);
  let src = fs::read_to_string(path)
    .expect("Couldn't read specified file!");

  println!("{}", src);
  
  let lexer = Lexer::new(src);

  lexer.lex().into_iter()
    .for_each(|a|{
      println!("{:?},", a);
    });
}
