pub enum Stmt{
  Block(Vec<Stmt>),
  Assignment(String, Expr),
}

pub enum Expr{
  BinaryExpr(Box<Expr>, BinOp, Box<Expr>),
  UnaryExpr(UnOp, Box<Expr>),
  Number(f32),
}

pub enum BinOp{
  Plus,
  Minus,
  Asterisk,
  FSlash,
}

pub enum UnOp{
  Minus,
}