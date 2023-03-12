use std::fmt::Debug;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenKind{
  Identifier(String),
  Number(f32),
  StringLiteral(String),
  
  ParenOpen,
  ParenClose,

  BraceOpen,
  BraceClose,

  Semicolon,

  Assignment,

  Let,
  FunctionDec,

  EOF,
}

pub struct Token{
  kind: TokenKind,
  line: usize,
  col : usize,
}

impl Token{
  pub fn new(kind: TokenKind, line: usize, col: usize) -> Token{
    Token{
      kind,
      line,
      col,
    }
  }
}

impl Debug for Token{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, 
      "[{:?} on {}, {}]",
        self.kind,
        self.line,
        self.col,
    )
  }
}
