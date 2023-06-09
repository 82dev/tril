use std::{println, rc::Rc, cell::RefCell};

use crate::{token::{Token, TokenKind}, nodes::{UnOp, BinOp, TopLevel, Statement, FunctionCall, Expression, Literal,}, types::{Type, FunctionType, PrimitiveType}};

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

  pub fn parse(&mut self) -> Vec<TopLevel>{
    let mut nodes = vec![];

    while self.tokens.get(self.current+1).is_some(){
      nodes.push(self.parse_top());
    }

    nodes
  }

  fn parse_top(&mut self) -> TopLevel{
    match self.advance(){
      TokenKind::FunctionDec => self.parse_fn(),
      TokenKind::Extern => self.parse_extern(),
      err => panic!("Unexpected token: {:?}", err)
    }
  }

  fn parse_statement(&mut self) -> Statement{
    match self.advance(){
      TokenKind::Let => self.parse_let(),
      TokenKind::Return => self.parse_return(),
      TokenKind::Identifier(id) => self.parse_ident(id),
      TokenKind::If => self.parse_if(),
      TokenKind::While => self.parse_while(),
      err => panic!("Unexpected token: {:?}", err)
    }
  }

  fn parse_while(&mut self) -> Statement{
    let e = self.parse_expr();
    self.expect(TokenKind::BraceOpen);
    Statement::While(e, self.parse_box_block())
  }

  fn parse_if(&mut self) -> Statement{
    let e = self.parse_expr();
    self.expect(TokenKind::BraceOpen);
    let ifbody = self.parse_box_block();

    let mut elsebody = vec![];

    if self.match_curr(TokenKind::Else){
      self.expect(TokenKind::BraceOpen);
      elsebody = self.parse_box_block();
    }

    Statement::If(e, ifbody, elsebody)
  }

  fn parse_ident(&mut self, id: String) -> Statement{
    match self.curr(){
      TokenKind::ParenOpen => {
        let a = self.parse_args();
        self.expect(TokenKind::Semicolon);
        Statement::FnCall(FunctionCall::new(id, a))
      },
      TokenKind::Assignment => {
        self.advance();
        let e = self.parse_expr();
        self.expect(TokenKind::Semicolon);
        Statement::Mutate(id, e)
      },
      _ => panic!("Unexpected Identifier: {id}"),
    }
  }

  fn parse_return(&mut self) -> Statement{
    if self.match_curr(TokenKind::Semicolon){
      return Statement::Return(None);
    }
    
    let e = self.parse_expr();
    self.expect(TokenKind::Semicolon);
    Statement::Return(Some(e))
  }

  fn parse_extern(&mut self) -> TopLevel{
    let name = self.expect_id().unwrap();
    let mut params = vec![];

    self.expect(TokenKind::ParenOpen);

    while !self.match_curr(TokenKind::ParenClose){
      loop{
        let t = self.expect_type().unwrap();
        params.push(Box::new(t));

        if !self.match_curr(TokenKind::Comma){
          break;
        }
      } 
    }

    let mut ret = None;

    if self.match_curr(TokenKind::MapsTo){
      ret = Some(Box::new(self.expect_type().unwrap()));
    }

    self.expect(TokenKind::Semicolon);

    TopLevel::Extern(
      name,
      FunctionType::new(ret, params)
    )
  }

  fn parse_fn(&mut self) -> TopLevel{
    let name = self.expect_id().unwrap();
    let params = self.parse_params();
    let mut t = None;

    if self.match_curr(TokenKind::MapsTo){
      t = Some(Box::new(self.expect_type().unwrap()));
    }

    self.expect(TokenKind::BraceOpen);
    let block = self.parse_block();

    TopLevel::FnDecl(
      name,
      FunctionType::new(t, params.1),
      params.0,
      block
    )
  }

  fn parse_params(&mut self) -> (Vec<String>, Vec<Box<Type>>){
    self.expect(TokenKind::ParenOpen);

    let mut argtypes = vec![];
    let mut args = vec![];

    while !self.match_curr(TokenKind::ParenClose){
      loop {
        let name = self.expect_id().unwrap();
        self.expect(TokenKind::Colon);
        let t = self.expect_type().unwrap();
        argtypes.push(Box::new(t));
        args.push(name);
        if !self.match_curr(TokenKind::Comma){
          break;
        }
      }
    }

    assert_eq!(args.len(), argtypes.len());
    (args, argtypes)
  }
 
  fn parse_args(&mut self) -> Vec<Expression>{
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

  fn parse_block(&mut self) -> Vec<Statement>{
    let mut stmts = vec![];
    while !self.match_curr(TokenKind::BraceClose) {
      stmts.push(self.parse_statement());
    }
    stmts
  }

  fn parse_box_block(&mut self) -> Vec<Box<Statement>>{
    let mut stmts = vec![];
    while !self.match_curr(TokenKind::BraceClose) {
      stmts.push(Box::new(self.parse_statement()));
    }
    stmts
  }

  fn parse_let(&mut self) -> Statement{
    let name = self.expect_id().unwrap();
    let mut t = Type::Unknown;
    //TODO:
    if self.match_curr(TokenKind::Colon){
      t = self.expect_type().unwrap();
    }
    self.expect(TokenKind::Assignment);
    let expr = self.parse_expr();
    self.expect(TokenKind::Semicolon);

    let t = RefCell::new(t);

    Statement::Assignment(name, t, expr)
  }

  fn parse_expr(&mut self) -> Expression{
    self.parse_equality()
  }

  fn parse_equality(&mut self) -> Expression{
    let mut lhs = self.parse_comparision();

    if self.match_curr(TokenKind::EqualTo){
      lhs = Expression::BinExpr(
        BinOp::Equal,
        Box::new(lhs),
        Box::new(self.parse_equality()),
        RefCell::new(Type::Primitive(PrimitiveType::Bool)),
      );
    }

    if self.match_curr(TokenKind::NotEqualTo){
      lhs = Expression::BinExpr(
        BinOp::NEqual,
        Box::new(lhs),
        Box::new(self.parse_equality()),
        RefCell::new(Type::Primitive(PrimitiveType::Bool)),
      );
    }

    return lhs;
  }

  fn parse_comparision(&mut self) -> Expression{
    let mut lhs = self.parse_term();

    if self.match_curr(TokenKind::LessThan){
      lhs = Expression::BinExpr(
        BinOp::Lesser,
        Box::new(lhs),
        Box::new(self.parse_comparision()),
        RefCell::new(Type::Primitive(PrimitiveType::Bool)),
      );
    }

    if self.match_curr(TokenKind::LessThanEqualTo){
      lhs = Expression::BinExpr(
        BinOp::LEq,
        Box::new(lhs),
        Box::new(self.parse_comparision()),
        RefCell::new(Type::Primitive(PrimitiveType::Bool)),
      );
    }
    if self.match_curr(TokenKind::GreaterThan){
      lhs = Expression::BinExpr(
        BinOp::Greater,
        Box::new(lhs),
        Box::new(self.parse_comparision()),
        RefCell::new(Type::Primitive(PrimitiveType::Bool)),
      );
    }
    if self.match_curr(TokenKind::GreaterThanEqualTo){
      lhs = Expression::BinExpr(
        BinOp::GEq,
        Box::new(lhs),
        Box::new(self.parse_comparision()),
        RefCell::new(Type::Primitive(PrimitiveType::Bool)),
      );
    }

    return lhs;
  }

  fn parse_term(&mut self) -> Expression{
    let mut expr = self.parse_factor();

    if self.match_curr(TokenKind::Plus){
      expr = Expression::BinExpr(
        BinOp::Add,
        Box::new(expr),
        Box::new(self.parse_term()),
        RefCell::new(Type::Unknown),
      );
    }
    if self.match_curr(TokenKind::Minus){
      expr = Expression::BinExpr(
        BinOp::Sub,
        Box::new(expr),
        Box::new(self.parse_term()),
        RefCell::new(Type::Unknown)
      );
    }

    expr
  }

  fn parse_factor(&mut self) -> Expression{
    let mut expr = self.parse_unary();

    if self.match_curr(TokenKind::Asterisk){
      expr = Expression::BinExpr(
        BinOp::Mul,
        Box::new(expr),
        Box::new(self.parse_unary()),
        RefCell::new(Type::Unknown),
      );
    }
    if self.match_curr(TokenKind::FSlash){
      expr = Expression::BinExpr(
        BinOp::Div,
        Box::new(expr),
        Box::new(self.parse_unary()),
        RefCell::new(Type::Unknown),
      );
    }

    expr
  }

  fn parse_unary(&mut self) -> Expression{
    if self.match_curr(TokenKind::Minus){
      return Expression::UnaryExpr(
        UnOp::ArithmeticNeg,
        Box::new(self.parse_expr()),
        RefCell::new(Type::Unknown),
      );
    }

    match self.curr(){
      TokenKind::Identifier(_) => {
        let n = self.expect_id().unwrap();
        match self.curr(){
          TokenKind::ParenOpen => {
            let a = self.parse_args();
            Expression::FnCall(FunctionCall::new(n, a))
          },
          _ => Expression::Variable(n, Type::Unknown)
        }
      },

      TokenKind::True => {self.advance(); Expression::Literal(Literal::Bool(true))},
      TokenKind::False => {self.advance(); Expression::Literal(Literal::Bool(false))},
      TokenKind::Float(_) => {Expression::Literal(Literal::Float(self.expect_float().unwrap()))},
      TokenKind::Int(_) => {Expression::Literal(Literal::Int(self.expect_int().unwrap()))},
      TokenKind::StringLiteral(_) => {Expression::Literal(Literal::String(self.expect_str().unwrap()))},
      TokenKind::ParenOpen => {
        self.advance();
        let e = self.parse_expr();
        self.expect(TokenKind::ParenClose);
        e
      },
      err => {panic!("Error parsing expr: {:?}", err)}
    }
  }

  fn expect(&mut self, kind: TokenKind){
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
  
  fn expect_float(&mut self) -> Option<f32>{
    match self.curr(){
      TokenKind::Float(i) => {
        self.advance();
        Some(i)
      },
      _ => None
    }
  }

  fn expect_int(&mut self) -> Option<i32>{
    match self.curr(){
      TokenKind::Int(i) => {
        self.advance();
        Some(i)
      }
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