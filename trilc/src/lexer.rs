//File -> Compiler -> String -> Lexer -> Tokens -> Parser -> Semantic Checker -> LLVM -> Exec -> :)

/*
  Eventual Rewrite Notes(TODO):
  advance returns Option(char)
  Whitespace and comment tokens
  string returns bool for termination

  have a look at rustc_lexer
*/

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer{
  pub source: Vec<char>,
  tokens: Vec<Token>,
  line: u32,
  col: u32,
  current: usize,
  len: usize,
}

impl Lexer{
  pub fn new(source: String) -> Lexer{
    let len = source.len();
    let source = source.chars().collect();
    Lexer{
      source,
      tokens: vec![],
      line: 1,
      col: 1,
      current: 0,
      len
    }
  }
  
  pub fn lex(mut self) -> Vec<Token>{
    while !self.is_at_end(){
      self.tokenize();
    }
    
    self.tokens
  }

  //This function should tokenize the token starting from the cursor(self.current) 
  fn tokenize(&mut self){
    match self.advance(){
      '\n' => {self.col = 1; self.line += 1;},
      '{' => self.add_token(TokenKind::BraceOpen),
      '}' => self.add_token(TokenKind::BraceClose),
      '(' => self.add_token(TokenKind::ParenOpen),
      ')' => self.add_token(TokenKind::ParenClose),
      ';' => self.add_token(TokenKind::Semicolon),
      ':' => self.add_token(TokenKind::Colon),
      '+' => self.add_token(TokenKind::Plus),
      '-' => self.add_token(TokenKind::Minus),
      '*' => self.add_token(TokenKind::Asterisk),
      '/' => {
        if self.source[self.current] == '/'{
          while self.advance() != '\n' && !self.is_at_end(){}
          self.line += 1;
          self.col = 1;
        }
        else{self.add_token(TokenKind::FSlash)}
      },
      '=' => {let k = self.match_curr('=', TokenKind::EqualTo, TokenKind::Assign); self.add_token(k)},
      '!' => {let k = self.match_curr('=', TokenKind::NotEqualTo, TokenKind::Bang); self.add_token(k)},
      '<' => {let k = self.match_curr('=', TokenKind::LessThanEqualTo, TokenKind::LessThan); self.add_token(k)},
      '>' => {let k = self.match_curr('=', TokenKind::GreaterThanEqualTo, TokenKind::GreaterThan); self.add_token(k)},

      '"' => self.string(),
      
      c => {
        if c.is_whitespace(){return}

        if c.is_alphabetic(){self.identifier();}

        if c.is_ascii_digit(){self.number();}
      }
    }
  }

  
  fn number(&mut self){
    let last = self.current - 1;
    while !self.is_at_end() && self.source[self.current].is_ascii_digit(){
      self.advance();
    }

    if !self.is_at_end(){
      if self.source[self.current] == '.'{
        self.advance();
        while !self.is_at_end() && self.source[self.current].is_ascii_digit(){
          self.advance();
        }

        let s: String = self.source[last..self.current].iter().collect();
        let num: f64 = s.parse().expect(format!("Couldn't parse number at line:{}, col:{}", self.line, self.col).as_str());
    
        self.add_token(TokenKind::FloatLiteral(num))
      }
      else{
        let s: String = self.source[last..self.current].iter().collect();
        let num: i64 = s.parse().expect(format!("Couldn't parse number at line:{}, col:{}", self.line, self.col).as_str());
    
        self.add_token(TokenKind::IntLiteral(num))
      }
    }
  }

  fn identifier(&mut self){
    let last = self.current - 1;
    while !self.is_at_end() && self.source[self.current].is_alphanumeric(){self.advance();}
    let s: String = self.source[last..self.current].iter().collect();
    self.add_token(
    match s.as_str(){
      "let" => TokenKind::Let,
      "if" => TokenKind::If,
      "else" => TokenKind::Else,
      "ret" => TokenKind::Ret,
      "while" => TokenKind::While,
      _ => TokenKind::Identifier(s)
    })
  }

  fn string(&mut self){
    let mut result = String::new();
    let last = self.current;
    while self.source[self.current] != '"'{
      if self.source[self.current] == '\\'{
        self.advance();
        result.push(
          match self.advance(){
            'n' => '\n',
            '\\' => '\\',
            '"' => '"',
            err => panic!("Can't escape '{err}'"),
          }
        );
        continue;
      }
      
      result.push(self.advance());
      if self.is_at_end(){
        let src: String = self.source.iter().collect();
        panic!("{}", format!("Unterminating string at line: {}, col: {}", self.line, self.col));
      }
    }
    self.advance();
    self.add_token(TokenKind::StringLiteral(result));
  }

  //return k1 if current == c else k2
  fn match_curr(&mut self, c: char, k1: TokenKind, k2: TokenKind) -> TokenKind{
    match self.source[self.current]{
      k if k == c => {self.advance(); k1},
      _ => k2
    }
  }

  fn add_token(&mut self, kind: TokenKind){
    self.tokens.push(Token::new(kind, self.line, self.col - 1));
  }

  fn advance(&mut self) -> char{
    self.col += 1;
    self.current += 1;
    self.source[self.current - 1]
  }

  fn is_at_end(&self) -> bool{
    self.current >= self.len
  }
}
