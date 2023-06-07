use std::fmt::Debug;

use crate::types::Type;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenKind{
  Identifier(String),
  Float(f32),
  Int(i32),
  StringLiteral(String),

  Type(Type),
  
  ParenOpen,
  ParenClose,

  BraceOpen,
  BraceClose,

  Colon,
  Semicolon,

  Comma,

  Assignment,

  Plus,
  Minus,
  Asterisk,
  FSlash,

  Bang,

  EqualTo,
  NotEqualTo,

  LessThanEqualTo,
  LessThan,
  GreaterThanEqualTo,
  GreaterThan,

  If,
  Else,

  While,

  Let,
  FunctionDec,
  MapsTo,

  Return,

  True,
  False,

  Extern,

  // EOF,
}

pub struct Token{
  pub kind: TokenKind,
  pub line: usize,
  pub col : usize,
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
