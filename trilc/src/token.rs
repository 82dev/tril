#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenKind{
  BraceOpen,
  BraceClose,
  ParenOpen,
  ParenClose,

  Semicolon,

  Plus,
  Minus,
  FSlash,
  Asterisk,

  Assign,

  Colon,

  EqualTo,
  NotEqualTo,
  LessThan,
  LessThanEqualTo,
  GreaterThan,
  GreaterThanEqualTo,

  Bang,

  Let,
  If,
  Else,

  Identifier(String),
  IntLiteral(i64),
  FloatLiteral(f64),
  StringLiteral(String),
}

#[derive(Debug)]
pub struct Token{
  kind: TokenKind,
  line: u32,
  col: u32,
}

impl Token{
  pub fn new(kind: TokenKind, line: u32, col: u32) -> Token{
    Token{
      kind,
      line,
      col
    }
  }
}
