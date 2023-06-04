use crate::types::{StructType, FunctionType, Type};

#[derive(Debug)]
#[derive(Clone)]
pub enum TopLevel{
  FnDecl(String, FunctionType, Vec<String>, Vec<Statement>),
  Extern(String, FunctionType),
  StructDecl(StructType)
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Statement{
  If(Expression, Vec<Box<Statement>>, Option<Vec<Box<Statement>>>),
  Assignment(String, Type, Expression),
  FnCall(FunctionCall),
  Return(Option<Expression>),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Expression{
  BinExpr(BinOp, Box<Expression>, Box<Expression>, Type),
  UnaryExpr(UnOp, Box<Expression>, Type),
  Literal(Literal),
  FnCall(FunctionCall),
  Variable(String, Type),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Literal{
  Int(i32),
  Float(f32),
  String(String),
  Bool(bool),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct FunctionCall{
  pub name: String,
  pub args: Vec<Expression>,
}

impl FunctionCall{
  pub fn new(name: String, args: Vec<Expression>) -> Self{
    FunctionCall{
      name,
      args,
    }
  }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum BinOp{
  Add,
  Sub,
  Mul,
  Div,

  Equal,
  NEqual,

  Lesser,
  LEq,
  Greater,
  GEq,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum UnOp{
  ArithmeticNeg,
}

impl From<Box<Expression>> for Expression{
  fn from(value: Box<Expression>) -> Self {
    *value
  }
}
