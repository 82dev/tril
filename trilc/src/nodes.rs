use crate::types::{FuncType, Type};

#[derive(Debug)]
#[derive(Clone)]
pub struct Call(pub String,pub Vec<Expr>);
#[derive(Debug)]
#[derive(Clone)]
pub struct Var(pub String, pub Type);

#[derive(Debug)]
#[derive(Clone)]
pub enum Stmt{
  Assignment(Var, Expr),
  FnDef(String, Vec<Var>, Vec<Stmt>, Type),
  Return(Expr)
}

#[derive(Debug)]
#[derive(Clone,)]
pub enum Expr{
  BinaryExpr(Box<Expr>, BinOp, Box<Expr>),
  UnaryExpr(UnOp, Box<Expr>),
  Number(f32),
  Var(Var),
  String(String),
  FnCall(Call),
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum BinOp{
  Plus,
  Minus,
  Asterisk,
  FSlash,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum UnOp{
  Minus,
}

impl From<Box<Expr>> for Expr{
  fn from(value: Box<Expr>) -> Self {
    *value
  }
}
