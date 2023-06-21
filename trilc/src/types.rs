use std::collections::HashMap;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Type{
  Primitive(PrimitiveType),
  Func(FunctionType),
  Struct(StructType),
  Void,
  Identifier(String),
  Unknown,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum PrimitiveType{
  Int,
  Float,
  String,
  Bool,
  Array(Box<Type>, Option<u32>),
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct StructType{
  pub contents: Vec<(String, Box<Type>)>,
}

impl StructType{
  pub fn new(contents: Vec<(String, Box<Type>)>) -> Self{
    Self{
      contents
    }
  }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct FunctionType{
  pub ret: Option<Box<Type>>,
  pub params: Vec<Box<Type>>,
}

impl FunctionType{
  pub fn new(ret: Option<Box<Type>>, params: Vec<Box<Type>>) -> Self{
    Self{
      ret,
      params,
    }
  }
}

impl From<Box<Type>> for Type{
  fn from(value: Box<Type>) -> Self {
    *value
  }
}
