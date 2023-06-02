use std::{collections::HashMap, println};

use crate::{types::{Type, FunctionType}, nodes::TopLevel};

pub struct SemanticAnalyzer{
  variables: HashMap<String, Type>,
  functions: HashMap<String, FunctionType>,
  tree: Vec<TopLevel>,
}

impl SemanticAnalyzer{
  pub fn new(tree: Vec<TopLevel>) -> Self{
    Self{
      variables: HashMap::new(),
      functions: HashMap::new(),
      tree
    }
  }

  pub fn analyze(mut self) -> Vec<TopLevel>{
    let mut binding = self.tree.get(0).unwrap().to_owned();
    if let TopLevel::FnDecl(ref mut name, _, _, _) = binding{
      name.push_str("aaa");
      println!("\n{}\n", name);
      println!("{:?}", binding);
    }
    
    self.tree
  }
}