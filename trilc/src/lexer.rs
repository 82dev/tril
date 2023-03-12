use crate::token::{Token, TokenKind};

pub struct Lexer{
  source: Vec<char>,

  last  : usize,
  pos   : usize,

  line  : usize,
  col   : usize,

  tokens: Vec<Token>,
}

impl Lexer{
  pub fn new(source: String) -> Lexer{
    Lexer{
      source: source.chars().collect(),
      last: 0,
      pos: 0,
      line: 1,
      col: 1,
      tokens: vec![],
    }
  }
}

impl Lexer{
  pub fn tokenize(mut self) -> Vec<Token>{
    while !self.is_at_end(){
      self.scan_token();
    }
    self.add_token(TokenKind::EOF);

    return self.tokens;
  }

  fn scan_token(&mut self){
    match self.advance(){
      ' ' | '\r' | '\t' => (),
      '\n' => {
        self.line += 1;
        self.col = 1;
      },

      '(' => self.add_token(TokenKind::ParenOpen),
      ')' => self.add_token(TokenKind::ParenClose),

      '{' => self.add_token(TokenKind::BraceOpen),
      '}' => self.add_token(TokenKind::BraceClose),

      ';' => self.add_token(TokenKind::Semicolon),

      '=' => self.add_token(TokenKind::Assignment),

      '"' => self.string(),
      
      c => {
        if c.is_alphabetic(){
          self.identifier();
        }
        else if c.is_ascii_digit(){
          self.number();
        }
      }
    }
    self.last = self.pos;
  }

  fn string(&mut self){
    while !self.is_at_end(){
      if self.advance() == '"' {
        break;
      }
    }
    if self.is_at_end() {
      panic!("{}", format!("Unterminating string at line:{}, col: {}", self.line, self.col));
    }

    self.add_token(TokenKind::StringLiteral(
      self.source[self.last + 1..self.pos - 1].iter().collect()
    ))
  }

  fn identifier(&mut self){
    while !self.is_at_end() && self.source[self.pos].is_ascii_alphanumeric(){
      self.advance();
    }

    let s: String = self.source[self.last..self.pos].iter().collect();

    let kind: TokenKind = match s.as_str(){
      "let" => TokenKind::Let,
      "fn" => TokenKind::FunctionDec,
      
      _ => TokenKind::Identifier(s)
    }; 

    self.add_token(kind)
  }


  fn number(&mut self){
    while !self.is_at_end() && self.source[self.pos].is_ascii_digit(){
      self.advance();
    }

    if !self.is_at_end(){
      if self.source[self.pos] == '.'{
        self.advance();
        while !self.is_at_end() && self.source[self.pos].is_ascii_digit(){
          self.advance();
        }
      }
    }
    
    let s: String = self.source[self.last..self.pos].iter().collect();
    let num: f32 = s.parse().expect(format!("Couldn't parse number at line:{}, col:{}", self.line, self.col).as_str());
    
    self.add_token(TokenKind::Number(num))
  }

  fn add_token(&mut self, kind: TokenKind){
    let t = Token::new(kind, self.line, self.col - 1);
    self.tokens.push(t)
  }

  fn advance(&mut self) -> char{
    self.pos += 1;
    self.col += 1;
    
    self.source[self.pos - 1]
  }

  fn peek_next(&self) -> char{
    self.source[self.pos + 1]
  }

  fn is_at_end(&self) -> bool{
    self.pos == self.source.len() as usize
  }
}