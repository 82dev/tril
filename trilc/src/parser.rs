use crate::{token::{Token, TokenKind}, node::{Stmt, Expr}};

pub struct Parser{
  tokens: Vec<Token>,
  current: usize,
}

impl Parser{
  pub fn new(tokens: Vec<Token>) -> Parser{
    Parser{
      tokens,
      current: 0
    }
  }

  pub fn parse(mut self) -> Vec<Stmt>{
    let mut nodes = vec![];

    while self.tokens.get(self.current + 1).is_some(){
      nodes.push(self.parse_stmt());
    }

    nodes
  }

  fn parse_stmt(&mut self) -> Stmt{
    match self.advance(){
      TokenKind::Let => self.parse_let(),
      TokenKind::Ret => self.parse_ret(),
      TokenKind::If  => self.parse_if() ,
      err => panic!("Unexpected token!: {:?}", err)
    }
  }
  
  //"let" ID (':' ID)? '=' EXPR ';'
  fn parse_let(&mut self) -> Stmt{
    let id = self.expect_id();
    let mut r#type = None;
    if self.check_curr(TokenKind::Colon){
      r#type = Some(self.expect_id());
    }
    self.expect(TokenKind::Assign);

    let expr = self.parse_expr();

    self.expect(TokenKind::Semicolon);

    Stmt::Assign{
      name: id,
      val: expr,
      type_name: r#type,
      r#type: None
    }
  }
  
  //"ret" EXPR ';'
  fn parse_ret(&mut self) -> Stmt{
    if self.check_curr(TokenKind::Semicolon){
      return Stmt::Return{
        expr: None
      }
    }

    let expr = self.parse_expr();
    self.expect(TokenKind::Semicolon);
    Stmt::Return{
      expr: Some(expr)
    }
  }

  //"if" EXPR BLOCK ("else" BLOCK)?
  fn parse_if(&mut self) -> Stmt{
    //current should be on EXPR
    let expr: Expr = self.parse_expr();
    let block: Vec<Stmt> = self.parse_block();

    if self.check_curr(TokenKind::Else){
      let elseBlock = self.parse_block();
      Stmt::If{
        cond: expr,
        trueBranch: block,
        falseBranch: Some(elseBlock)
      }
    }
    else{
      Stmt::If{
        cond: expr,
        trueBranch: block,
        falseBranch: None
      }
    }
  }     

  fn parse_block(&mut self) -> Vec<Stmt>{
    self.expect(TokenKind::BraceOpen);
    let mut stmts = vec![];

    while !self.check_curr(TokenKind::BraceClose){
      stmts.push(self.parse_stmt());
    }

    stmts
  }

  //Lowest precedence comes first
  /*
    EXPR := TERM (+/-) EXPR
  */
  fn parse_expr(&mut self) -> Expr{
    self.parse_equality()
  }

  fn parse_equality(&mut self) -> Expr{
    let mut lhs = self.parse_comp();

    if self.check_curr(TokenKind::EqualTo){
      lhs = Expr::EqualTo(Box::new(lhs), Box::new(self.parse_equality()));
    }
    if self.check_curr(TokenKind::NotEqualTo){
      lhs = Expr::NotEqualTo(Box::new(lhs), Box::new(self.parse_equality()));
    }

    lhs
  }

  fn parse_comp(&mut self) -> Expr{
    let mut lhs = self.parse_term();

    if self.check_curr(TokenKind::LessThan){
      lhs = Expr::LessThan(Box::new(lhs), Box::new(self.parse_comp()));
    }
    if self.check_curr(TokenKind::LessThanEqualTo){
      lhs = Expr::LessThanEqualTo(Box::new(lhs), Box::new(self.parse_comp()));
    }
    if self.check_curr(TokenKind::GreaterThan){
      lhs = Expr::GreaterThan(Box::new(lhs), Box::new(self.parse_comp()));
    }
    if self.check_curr(TokenKind::GreaterThanEqualTo){
      lhs = Expr::GreaterThanEqualTo(Box::new(lhs), Box::new(self.parse_comp()));
    }

    lhs
  }

  fn parse_term(&mut self) -> Expr{
    let mut lhs = self.parse_factor();

    if self.check_curr(TokenKind::Plus){
      lhs = Expr::Add(Box::new(lhs), Box::new(self.parse_term()))
    }

    if self.check_curr(TokenKind::Minus){
      lhs = Expr::Sub(Box::new(lhs), Box::new(self.parse_term()))
    }

    lhs    
  }

  fn parse_factor(&mut self) -> Expr{
    let mut lhs = self.parse_unary();

    if self.check_curr(TokenKind::Asterisk){
      lhs = Expr::Mul(Box::new(lhs), Box::new(self.parse_term()))
    }

    if self.check_curr(TokenKind::FSlash){
      lhs = Expr::Div(Box::new(lhs), Box::new(self.parse_term()))
    }

    lhs    
  }

  fn parse_unary(&mut self) -> Expr{
    if self.check_curr(TokenKind::Minus){
      return Expr::Negate(Box::new(self.parse_expr()));
    }

    match self.advance(){
      TokenKind::Identifier(n) => {
        //The other arms are for fncall and array indexing
        match self.tokens[self.current]{
          _ => Expr::Variable(n)
        }
      },

      TokenKind::IntLiteral(i) => Expr::IntLiteral(i),
      TokenKind::FloatLiteral(f) => Expr::FloatLiteral(f),
      TokenKind::StringLiteral(s) => Expr::StringLiteral(s),
      
      _ => panic!("Error parsing expr")
    }
  }

  fn expect_id(&mut self) -> String{
    match self.tokens[self.current].kind.clone(){
      TokenKind::Identifier(s) => {self.advance(); s},
      _ => panic!()
    }
  }

  //Checks if curr token is of kind k, if so consume and return true,
  //else, return false
  fn check_curr(&mut self, k: TokenKind) -> bool{
    match self.tokens.get(self.current){
      Some(t) => {
        if t.kind == k{
          self.advance();
          true
        }
        else {false}
      }
      None => false,
    }
  }

  fn expect(&mut self, k: TokenKind){
    match self.check_curr(k){
      true => (),
      false => panic!("Unexpected token: {:?}", self.tokens[self.current])
    }
  }

  fn advance(&mut self) -> TokenKind{
    self.current += 1;
    self.tokens[self.current - 1].kind.clone()
  }
}
