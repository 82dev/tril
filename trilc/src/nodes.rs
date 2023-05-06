#[derive(Debug)]
pub enum Stmt{
  Block(Vec<Stmt>),
  Assignment(String, Expr),
}

#[derive(Debug)]
pub enum Expr{
  BinaryExpr(Box<Expr>, BinOp, Box<Expr>),
  UnaryExpr(UnOp, Box<Expr>),
  Number(f32),
  Var(String)
}

#[derive(Debug)]
pub enum BinOp{
  Plus,
  Minus,
  Asterisk,
  FSlash,
}

#[derive(Debug)]
pub enum UnOp{
  Minus,
}