use crate::token::Token;

pub struct Parser{
  tokens: Vec<Token>,
  current: usize,
}

impl Parser{
  pub fn new(tok: Vec<Token>) -> Parser{
    Parser{
      tokens: tok,
      current: 0
    }
  }

  pub fn parse(&mut self) -> Vec<Stmt>{
    let ast = Vec::new();

    ast
  }
}