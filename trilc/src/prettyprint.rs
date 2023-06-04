// use std::{fmt::Debug, write};

// use crate::{nodes::{TopLevel, Statement, Expression, Literal, FunctionCall}, types::FunctionType};

// impl Debug for TopLevel{
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     match self{
//       TopLevel::FnDecl(name, ft, _, body) => write!(f, "\n{:?}[{:?}]{{\n  {:?}\n}}\n", name, ft, body),
//       TopLevel::Extern(name, ft) => write!(f, "\nextern {:?}[{:?}]\n", name, ft),
//       TopLevel::StructDecl(_) => todo!(),
//     }
//   }
// }

// impl Debug for Statement{
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     match self{
//       Statement::Assignment(name, ty, e) => write!(f, "{:?}: {:?} = {:?}\n", name, ty, e),
//       Statement::FnCall(fc) => write!(f, "{:?}(stmt)\n", fc),
//       Statement::Return(e) => write!(f, "{:?}", e),
//       Statement::If(cond, ifbody, elsebody) => write!(f, "if ({:?}) -> {{\n  {:?}\n}} else -> {{\n  {:?}\n}}\n", cond, ifbody, elsebody),
//     }
//   }
// }

// impl Debug for Expression{
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     match self{
//       Expression::Literal(lit) => write!(f, "{:?}", lit),
//       Expression::FnCall(fc) => write!(f, "{:?}", fc),
//       Expression::Variable(name, ty) => write!(f, "{:?}({:?})", name, ty),
//       Expression::UnaryExpr(uop, rhs, ty) => write!(f, "({:?}{:?})({:?})", uop, rhs, ty),
//       Expression::BinExpr(bop, lhs, rhs, ty) => write!(f, "({:?} {:?} {:?})({:?})", lhs, bop, rhs, ty),
//     }
//   }
// }

// impl Debug for Literal{
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     match self{
//       Literal::Int(i) => write!(f, "{}", i),
//       Literal::Float(fl) => write!(f, "{}", fl),
//       Literal::String(s) => write!(f, "{}", s),
//       Literal::Bool(b) => write!(f, "{}", b),
//     }
//   }
// }

// impl Debug for FunctionCall{
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "{:?}({:?})", self.name, self.args)
//   }
// }

// impl Debug for FunctionType{
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "({:?}) -> {:?}", self.params, self.ret)
//   }
// }