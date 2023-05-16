#[derive(Debug)]
#[derive(Clone)]
pub struct Call(pub String,pub Vec<Expr>);

#[derive(Debug)]
#[derive(Clone)]
pub enum Stmt{
  Assignment(String, Expr),
  FnDef(String, Vec<String>, Vec<Stmt>),
  FnCall(Call),
  Return(Expr)
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Expr{
  BinaryExpr(Box<Expr>, BinOp, Box<Expr>),
  UnaryExpr(UnOp, Box<Expr>),
  Number(f32),
  Var(String),
  String(String),
  FnCall(Call),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum BinOp{
  Plus,
  Minus,
  Asterisk,
  FSlash,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum UnOp{
  Minus,
}

impl From<Box<Expr>> for Expr{
  fn from(value: Box<Expr>) -> Self {
    *value
  }
}