mod token;
mod lexer;
mod parser;
mod nodes;
mod codegen;
mod types;
mod typefiller;
mod prettyprint;

use std::{env, fs, println, path::PathBuf};

use inkwell::context::Context;

use crate::{lexer::Lexer, parser::Parser, typefiller::TypeFiller};

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

  let mut path = PathBuf::from(file_path);
  path.set_extension("ll");

  let tok = Lexer::new(contents).tokenize();
  println!("{:?}\n\n", tok);
  let nodes = TypeFiller::new(Parser::new(tok).parse()).fill();
  println!("Typed: \n{:?}\n\n", nodes);

  // let context = Context::create();
  // let module = context.create_module(path.file_stem().unwrap().to_str().unwrap());
  // let builder = context.create_builder();  

  // println!("{:?}\n\n", codegen::CodeGenerator::new(&context, module, builder, nodes).generate(&path));
}
