use core::panic;

use crate::{token::{Token, TokenKind}, nodes::{Stmt, Expr, UnOp, BinOp, Call, Var}, types::Type};

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
    let mut nodes = vec![];

    while self.tokens.get(self.current+1).is_some(){
      nodes.push(self.parse_stmt());
    }

    nodes
  }

  fn parse_stmt(&mut self) -> Stmt{
    match self.advance(){
      TokenKind::Let => self.parse_let(),
      TokenKind::FunctionDec => self.parse_fn(),
      TokenKind::Return => self.parse_return(),
      err => panic!("Unexpected token: {:?}", err)
    }
  }

  fn parse_return(&mut self) -> Stmt{
    let e = self.parse_expr();
    self.expect(TokenKind::Semicolon);
    Stmt::Return(e)
  }

  fn parse_fn(&mut self) -> Stmt{
    let name = self.expect_id().unwrap();
    let params = self.parse_params();
    let mut t = Type::Void;

    if self.match_curr(TokenKind::MapsTo){
      t = self.expect_type().unwrap();
    }

    self.expect(TokenKind::BraceOpen);
    let block = self.parse_block();

    Stmt::FnDef(name, params, block, t)
  }

  fn parse_params(&mut self) -> Vec<Var>{
    self.expect(TokenKind::ParenOpen);
    let mut params = vec![];

    while !self.match_curr(TokenKind::ParenClose){
      loop {
        let name = self.expect_id().unwrap();
        self.expect(TokenKind::Colon);
        let t = self.expect_type().unwrap();
        params.push(Var(name, t));
        if !self.match_curr(TokenKind::Comma){
          break;
        }
      }
    }

    params
  }
 
  fn parse_args(&mut self) -> Vec<Expr>{
    self.expect(TokenKind::ParenOpen);
    let mut params = vec![];

    while !self.match_curr(TokenKind::ParenClose){
      loop {
        params.push(self.parse_expr());
        if !self.match_curr(TokenKind::Comma){
          break;
        }
      }
    }

    params
  }

  fn parse_block(&mut self) -> Vec<Stmt>{
    let mut stmts = vec![];
    while !self.match_curr(TokenKind::BraceClose) {
      stmts.push(self.parse_stmt());
    }
    stmts
  }

  fn parse_let(&mut self) -> Stmt{
    let name = self.expect_id().unwrap();
    let mut t = Type::Untyped;
    //TODO:
    if self.match_curr(TokenKind::Colon){
      t = self.expect_type().unwrap();
    }
    self.expect(TokenKind::Assignment);
    let expr = self.parse_expr();
    self.expect(TokenKind::Semicolon);

    Stmt::Assignment(Var(name, t), expr)
  }

  fn parse_expr(&mut self) -> Expr{
    self.parse_term()
  }

  fn parse_term(&mut self) -> Expr{
    let mut expr = self.parse_factor();

    if self.match_curr(TokenKind::Plus){
      expr = Expr::BinaryExpr(
        Box::new(expr),
        BinOp::Plus,
        Box::new(self.parse_term()));
    }
    if self.match_curr(TokenKind::Minus){
      expr = Expr::BinaryExpr(
        Box::new(expr),
        BinOp::Minus,
        Box::new(self.parse_term()));
    }

    expr
  }

  fn parse_factor(&mut self) -> Expr{
    let mut expr = self.parse_unary();

    if self.match_curr(TokenKind::Asterisk){
      expr = Expr::BinaryExpr(
        Box::new(expr),
        BinOp::Asterisk,
        Box::new(self.parse_unary()));
    }
    if self.match_curr(TokenKind::FSlash){
      expr = Expr::BinaryExpr(
        Box::new(expr),
        BinOp::FSlash,
        Box::new(self.parse_unary()));
    }

    expr
  }

  fn parse_unary(&mut self) -> Expr{
    if self.match_curr(TokenKind::Minus){
      return Expr::UnaryExpr(
        UnOp::Minus,
        Box::new(self.parse_expr())
      );
    }

    //TODO: '('expr')'
    //TODO: Function Call
    match self.curr(){
      TokenKind::Identifier(_) => {
        let n = self.expect_id().unwrap();
        if self.curr() == TokenKind::ParenOpen{
          let a = self.parse_args();
          return Expr::FnCall(Call(n, a));
        }
        Expr::Var(Var(n, Type::Untyped))
      },
      TokenKind::Number(_) => {Expr::Number(self.expect_num().unwrap())},
      TokenKind::StringLiteral(_) => {Expr::String(self.expect_str().unwrap())},
      TokenKind::ParenOpen => {
        self.advance();
        let e = self.parse_expr();
        self.expect(TokenKind::ParenClose);
        e
      }
      _ => {panic!()}
    }
  }

  fn expect(&mut self, kind: TokenKind){
    println!("{:?} {:?}", self.current, kind);
    if !self.match_curr(kind){
      panic!()
    }
  }

  //cant use match
  fn match_curr(&mut self, kind: TokenKind) -> bool{
    match self.curr(){
      tk if tk == kind => {self.advance(); true},
      _ => false 
    }
  }

  fn expect_type(&mut self) -> Option<Type>{
    match self.curr(){
      TokenKind::Type(t) => {
        self.advance();
        Some(t)
      },
      _ => None
    }
  }
  
  fn expect_num(&mut self) -> Option<f32>{
    match self.curr(){
      TokenKind::Number(num) => {
        self.advance();
        Some(num)
      },
      _ => None
    }
  }

  fn expect_id(&mut self) -> Option<String>{
    match self.curr(){
      TokenKind::Identifier(id) => {
        self.advance();
        Some(id)
      },
      _ => None
    }
  }

  fn expect_str(&mut self) -> Option<String>{
    match self.curr(){
      TokenKind::StringLiteral(s) => {
        self.advance();
        Some(s)
      },
      _ => None
    }
  }

  fn peek_next(&self) -> TokenKind{
    self.tokens[self.current + 1].kind.clone()
  }

  fn advance(&mut self) -> TokenKind{
    self.current += 1;
    self.tokens[self.current - 1].kind.clone()
  }

  fn curr(&self) -> TokenKind{
    //TODO
    self.tokens[self.current].kind.clone()
  }
}