use crate::{token::{Token, TokenKind}, nodes::{Stmt, Expr, UnOp, BinOp}};

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
      err => panic!("Unexpected token: {:?}", err)
    }
  }

  fn parse_let(&mut self) -> Stmt{
    let name = self.expect_id().unwrap();
    self.expect(TokenKind::Assignment);
    let expr = self.parse_expr();
    self.expect(TokenKind::Semicolon);

    Stmt::Assignment(name, expr)
  }

  fn parse_expr(&mut self) -> Expr{
    self.parse_term()
  }

  fn parse_term(&mut self) -> Expr{
    let mut expr = self.parse_factor();

    if self.expect(TokenKind::Plus){
      expr = Expr::BinaryExpr(
        Box::new(expr),
        BinOp::Plus,
        Box::new(self.parse_term()));
    }
    if self.expect(TokenKind::Minus){
      expr = Expr::BinaryExpr(
        Box::new(expr),
        BinOp::Minus,
        Box::new(self.parse_term()));
    }

    expr
  }

  fn parse_factor(&mut self) -> Expr{
    let mut expr = self.parse_unary();

    if self.expect(TokenKind::Asterisk){
      expr = Expr::BinaryExpr(
        Box::new(expr),
        BinOp::Asterisk,
        Box::new(self.parse_unary()));
    }
    if self.expect(TokenKind::FSlash){
      expr = Expr::BinaryExpr(
        Box::new(expr),
        BinOp::FSlash,
        Box::new(self.parse_unary()));
    }

    expr
  }

  fn parse_unary(&mut self) -> Expr{
    if self.expect(TokenKind::Minus){
      return Expr::UnaryExpr(
        UnOp::Minus,
        Box::new(self.parse_expr())
      );
    }

    //TODO: '('expr')'
    if let Some(num) = self.expect_num(){
      return Expr::Number(num);
    }

    if let Some(v) = self.expect_id(){
      return Expr::Var(v);
    }

    panic!()
  }

  fn expect(&mut self, kind: TokenKind) -> bool{
    match self.curr(){
      tk if tk == kind => {self.advance(); true},
      _ => false 
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