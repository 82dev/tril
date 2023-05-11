#[derive(Debug)]
pub struct Call(pub String,pub Vec<Expr>);

#[derive(Debug)]
pub enum Stmt{
  Assignment(String, Expr),
  FnDef(String, Vec<String>, Vec<Stmt>),
  FnCall(Call)
}

#[derive(Debug)]
pub enum Expr{
  BinaryExpr(Box<Expr>, BinOp, Box<Expr>),
  UnaryExpr(UnOp, Box<Expr>),
  Number(f32),
  Var(String),
  String(String),
  FnCall(Call),
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