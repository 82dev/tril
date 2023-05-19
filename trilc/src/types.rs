use std::{fmt::{Display, Debug}, write};
use crate::nodes::{BinOp, UnOp, Var, Call};

#[derive(Clone)]
#[derive(Debug)]
pub struct TypedCall(pub String, pub Vec<TypedExpr>, pub Type);

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Type{
  F32,
  String,
  Untyped,
  Void,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct FuncType{
  pub params: Vec<Type>,
  pub ret_type: Type,
}

#[derive(Clone)]
pub enum TypedAst{
  Assignment(String, TypedExpr, Type),
  //   name     paramaters   block      
  FnDef(String, Vec<Var>, Vec<TypedAst>, FuncType),
  //using just 'Return' causes some error idfk y
  TReturn(TypedExpr)
}

#[derive(Clone)]
pub enum TypedExpr{
  BinaryExpr(Box<TypedExpr>, Box<TypedExpr>, BinOp),
  UnaryExpr(UnOp, Box<TypedExpr>),
  Var(String, Type),
  String(String),
  Number(f32),
  FnCall(TypedCall),
}

impl<'a> PartialEq<&'a Type> for Type{
  fn eq(&self, other: &&'a Type) -> bool {
    self == *other
  }
}
impl<'a> PartialEq<Type> for &'a Type{
  fn eq(&self, other: &Type) -> bool {
    *self == other
  }
}

impl Debug for TypedAst{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self{
      TypedAst::Assignment(name, texpr, t) => write!(f, "{}: {:?} = {:?}", name, t, texpr),
      TypedAst::TReturn(texpr) => write!(f, "ret ({:?})", texpr),
      TypedAst::FnDef(name, params, body, fntype) => {
        write!(f, "\n {}: {:?} -> {:?}\n {}: {{{:?}}}\n", name, params, fntype.ret_type, name, body)
      }
    }    
  } 
}

impl Debug for TypedExpr{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self{
      Self::Number(num) => write!(f, "{}", num),
      Self::String(s) => write!(f, "\"{}\"", s),
      Self::Var(name, t) => write!(f, "{}: {:?}", name, t),
      Self::UnaryExpr(uop, texpr) => {
        let op = match uop{
          UnOp::Minus => "-"
        };
        write!(f, "{}({:?})", op, *texpr)
      },
      Self::BinaryExpr(lhs, rhs, bop) => {
        let op = match bop{
          BinOp::Plus => "+",
          BinOp::Minus => "-",
          BinOp::Asterisk => "*",
          BinOp::FSlash => "/",
        };
        write!(f, "({:?}) {} ({:?})", *lhs, op, *rhs)
      },
      Self::FnCall(call) => write!(f, "{}({:?})", call.0, call.1)
    }
  }
}