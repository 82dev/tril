use std::{collections::HashMap, assert_eq, println, matches, unreachable};

use crate::{nodes::{Stmt, Expr, BinOp, Var}, types::{FuncType, Type, TypedAst, TypedExpr, TypedCall}};

pub struct TypeChecker{
  ast: Vec<Stmt>,
  variables: HashMap<String, Type>,
  functions: HashMap<String, FuncType>
}

impl TypeChecker{
  pub fn new(ast: Vec<Stmt>) -> Self{
    let mut functions: HashMap<String, FuncType> = HashMap::new();
    functions.insert("printf_ascii".into(), FuncType{
      params: vec![Type::F32],
      ret_type: Type::F32,
    });
    TypeChecker{
      ast,
      variables: HashMap::new(),
      functions,
    }
  }
  
  pub fn get_typed_ast(mut self) -> Result<Vec<TypedAst>, String>{
    let mut tast = vec![];

    for i in 0..self.ast.len(){
      tast.push(self.type_stmt(self.ast[i].to_owned()));
    }
    
    Ok(tast)
  }

  fn type_stmt(&mut self, stmt: Stmt) -> TypedAst{
    match stmt{
      Stmt::Assignment(var, expr) => self.type_ass(var.0, expr, var.1),
      Stmt::FnDef(name, params, body, t) => self.type_fn(name, params, body, t),
      Stmt::Return(expr) => unreachable!()
    }
  }

  fn type_ass(&mut self, name: String, expr: Expr, t: Type) -> TypedAst{
    let te = self.type_expr(expr);
    let et = self.get_expr_type(&te);

    if t != Type::Untyped{
      assert_eq!(t, et);
    }

    if self.variables.contains_key(&name){
      assert_eq!(et, *self.variables.get(&name).unwrap());
      return TypedAst::Assignment(name, te, et)
    }

    self.variables.insert(name.clone(), et.clone());
    TypedAst::Assignment(name, te, et)
  }

  fn type_fn(&mut self, name: String, params: Vec<Var>, body: Vec<Stmt>, ret_type: Type) -> TypedAst{
    let mut tast = vec![];
    self.variables.clear();

    let mut functype = FuncType{
      params: vec![],
      ret_type
    };
    
    if params.len() > 0{
      for param in &params{
        self.variables.insert(param.0.clone(), param.1);
        functype.params.push(param.1);
      }
    }
    
    for stmt in body{
      match stmt{
        Stmt::Return(e) => {
          let e = self.type_expr(e);
          let t = self.get_expr_type(&e);
          assert_eq!(t, ret_type);
          tast.push(TypedAst::TReturn(e))
        }
        _ => tast.push(self.type_stmt(stmt))
      }
    }
    self.functions.insert(name.clone(), functype.clone());
    TypedAst::FnDef(name, params, tast, functype)
  }

  fn type_expr(&self, expr: Expr) -> TypedExpr{
    match expr{
      Expr::String(s) => TypedExpr::String(s.into()),
      //TODO: Proper errors
      Expr::Var(var) => {
        println!("{:?}", self.variables);
        TypedExpr::Var(var.0.to_string(), *self.variables.get(&var.0).unwrap())
       },
      Expr::Number(num) => TypedExpr::Number(num),
      Expr::UnaryExpr(uop, e) => TypedExpr::UnaryExpr(uop, Box::new(self.type_expr(*e))),
      Expr::BinaryExpr(lhs, bop, rhs) => TypedExpr::BinaryExpr(
        Box::new(self.type_expr(*lhs)),
        Box::new(self.type_expr(*rhs)),
        bop
      ),
      Expr::FnCall(call) => {
        //TODO: More info
        let name = call.0;
        let targs = call.1.iter().map(|e| self.type_expr(e.clone())).collect();
        let t = self.functions.get(&name).unwrap();

        TypedExpr::FnCall(TypedCall(name, targs, t.ret_type))
      }
    }
  }

  fn get_expr_type(&self, tex: &TypedExpr) -> Type{
    match tex{
      TypedExpr::Number(_) => Type::F32,
      TypedExpr::String(_) => Type::String,
      TypedExpr::Var(_, t) => *t,
      TypedExpr::UnaryExpr(_, te) => {
        assert_eq!(self.get_expr_type(&*te), Type::F32);
        Type::F32
      },
      TypedExpr::BinaryExpr(lhs, rhs, bop) => {
        let t1 = self.get_expr_type(&*lhs);
        let t2 = self.get_expr_type(&*rhs);
        assert_eq!(t1, t2);

        match bop{
          BinOp::Plus => {
            t1
          }
          _ => {
            assert_eq!(t1, Type::F32);
            t1
          }
        }
      },
      TypedExpr::FnCall(call) => {
        if !self.functions.contains_key(&call.0){
          panic!("Could not find function: {}", call.0);
        }

        let fntype = self.functions.get(&call.0).unwrap();

        assert_eq!(call.1.len(), fntype.params.len());

        for (i, param) in fntype.params.iter().enumerate(){
          assert_eq!(
            param,
            self.get_expr_type(&call.1[i])
          );
        }
        
        fntype.ret_type
      }
    }
  }
}