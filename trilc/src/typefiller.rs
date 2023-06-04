use std::{collections::HashMap, println};

use crate::{nodes::{TopLevel, Statement, Expression, FunctionCall, Literal}, types::{Type, FunctionType, PrimitiveType}};

pub struct TypeFiller{
  tree: Vec<TopLevel>,
  variables: HashMap<String, Type>,
  functions: HashMap<String, FunctionType>,
  current_ret: Option<Type>,
}

impl TypeFiller{
  pub fn new(tree: Vec<TopLevel>) -> TypeFiller{
    TypeFiller{
      tree,
      variables: HashMap::new(),
      functions: HashMap::new(),
      current_ret: None,
    }
  }

  pub fn fill(mut self) -> Vec<TopLevel>{
    for top in self.tree.to_owned(){
      match top{
        TopLevel::FnDecl(name, ft, _, _) => {
          if self.functions.insert(name.clone(), ft).is_some(){
            panic!("Cant redifine function!: {:?}", name);
          };
        },
        TopLevel::Extern(name, ft) => {self.functions.insert(name, ft);},
        _ => (),
      }
    }

    for top in self.tree.to_owned(){
      self.fill_top(top);
    }

    println!("{:?}", self.functions);
    
    self.tree
  }

  fn fill_top(&mut self, top: TopLevel){
    match top{
      TopLevel::FnDecl(name, ft, params, body) => self.fill_fn(name, ft, params, body),
      _ => (),
    }
  }

  fn fill_fn(&mut self, name: String, ft: FunctionType, params: Vec<String>, body: Vec<Statement>){
    self.current_ret = match ft.ret{
      Some(t)  => Some(*t),
      None => None,
    };
    self.variables.clear();
    for (i, p) in ft.params.iter().enumerate(){
      self.variables.insert(params[i].clone(), *p.clone());
    }
    
    for stmt in body{
      self.fill_stmt(stmt);
    }
  }

  fn fill_stmt(&mut self, mut stmt: Statement){
    match stmt{
      Statement::If(expr, ifbody, elsebody) => todo!(),
      Statement::Assignment(name, mut ty, expr) => self.fill_ass(name, ty, expr),
      Statement::FnCall(fc) => {self.check_call(fc);},
      Statement::Return(expr) => self.check_return(expr),
    }
  }

  fn fill_ass(&mut self, name: String, mut ty: Type, e: Expression){
    let mut t1 = self.get_expr_type(&e).expect(&format!("Could not determine type of expression: '{:?}'", e));
    if ty == Type::Unknown{
      ty = t1;
      self.variables.insert(name, ty);
      return;
    }

    if ty != t1{
      panic!("Cannot assign '{:?}' to variable '{:?}', which has type '{:?}'", name, e, ty);
    }
    self.variables.insert(name, ty);
  }

  fn check_call(&self, fc: FunctionCall) -> Option<Type>{
    let name = fc.name;
    let ft = self.functions.get(&name).expect(&format!("Could not find function: '{:?}'", name));
    let args = fc.args;

    if args.len() != ft.params.len(){
      panic!("Expected '{:?}' arguements but found '{:?}' at function call: '{:?}'", ft.params.len(), args.len(), name);
    }

    for (i, e) in args.iter().enumerate(){
      let et = self.get_expr_type(&e).expect(&format!("Could not determine type of expression: '{:?}'", e));
      let pt = ft.params[i].clone();
      if et != *pt{
        panic!("Expected {:?} but got {:?}", pt, et);
      }
    }

    match ft.ret.clone(){
      Some(t) => Some(*t),
      None => None,
    }
  }

  fn check_return(&self, expr: Option<Expression>){
    let et = 
      if let Some(expr) = expr{
        Some(self.get_expr_type(&expr).expect(&format!("Could not determine type of expression: '{:?}'", expr)))
      }
      else{
        None
      };
    //void returns
    if et != self.current_ret{
      panic!("Expected return type of '{:?}', but found '{:?}' instead.", self.current_ret, et);
    }
  }

  fn get_expr_type(&self, e: &Expression) -> Result<Type, ()>{
		match e{
			Expression::Variable(name, mut t) => {
        let t1 = self.variables.get(name)
          .expect(
            &format!("Variable '{:?}' not found.", name)).clone();
        if t == Type::Unknown{
          t = t1;
          Ok(t.clone())
        }
        else{
          Ok(t1)
        }
      },
			Expression::Literal(lit) => match lit{
				Literal::Int(_) => Ok(Type::Primitive(PrimitiveType::Int)),
				Literal::Float(_) => Ok(Type::Primitive(PrimitiveType::Float)),
				Literal::String(_) => Ok(Type::Primitive(PrimitiveType::String)),
				Literal::Bool(_) => Ok(Type::Primitive(PrimitiveType::Bool)),
			},
			Expression::FnCall(fcall) => {
        let ft = self.functions.get(&fcall.name)
          .expect(
            &format!("Could'nt find function '{:?}'", fcall.name));
        
        match ft.ret.clone(){
          Some(t) => Ok(*t),
          None => Ok(Type::Void),
        }
			},
			Expression::UnaryExpr(_, ex, mut ty) => {
				ty = self.get_expr_type(&ex).unwrap();
				Ok(ty)
			},
			Expression::BinExpr(_, lhs, rhs, mut ty) => {
				let lt = self.get_expr_type(&lhs).unwrap();
				assert_eq!(lt, self.get_expr_type(&rhs).unwrap());
				ty = lt;
				Ok(ty)
			}
		}
  }
}