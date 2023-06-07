use std::{collections::HashMap, println,  cell::RefCell, ptr::replace};

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

    let mut graft = self.tree.to_owned();
    
    for top in graft.iter_mut(){
      self.fill_top(top);
    }

    println!("{:?}\n\n", self.functions);
    
    graft
  }

  fn fill_top(&mut self, top: &mut TopLevel){
    match top{
      TopLevel::FnDecl(name, ft, params, body) => self.fill_fn(name, ft, params, body),
      _ => (),
    }
  }

  fn fill_fn(&mut self, name: &mut String, ft: &mut FunctionType, params: &mut Vec<String>, mut body: &mut Vec<Statement>){
    self.current_ret = match ft.ret.clone(){
      Some(t)  => Some(*t),
      None => None,
    };
    self.variables.clear();
    for (i, p) in ft.params.iter().enumerate(){
      self.variables.insert(params[i].clone(), *p.clone());
    }
    
    for stmt in body.iter_mut(){
      self.fill_stmt(stmt);
    }
  }

  fn fill_stmt(&mut self, mut stmt: &mut Statement){
    match stmt{
      Statement::While(expr, body) => self.fill_while(expr, body),
      Statement::If(expr, ifbody, elsebody) => self.fill_if(expr, ifbody, elsebody),
      Statement::Assignment(name, ty, expr) => {self.fill_ass(name, ty, expr)},
      Statement::Mutate(name, expr) => self.fill_mut(name, expr),
      Statement::FnCall(fc) => {self.check_call(fc);},
      Statement::Return(expr) => self.check_return(expr),
    }
  }

  fn fill_while(&mut self, e: &mut Expression, body: &mut Vec<Box<Statement>>){
    assert_eq!(self.get_expr_type(e).unwrap(), Type::Primitive(PrimitiveType::Bool));
    //TODO: Scopes: FIXME:
    for s in body{
      self.fill_stmt(s);
    }
  }

  fn fill_mut(&self, name: &String, expr: &mut Expression) {
    let t1 = self.get_expr_type(expr).unwrap();
    let t2 = self.variables.get(name).expect("No variable {name} found. Perhaps you might like to use 'let'");
    assert_eq!(t1, *t2);
  }

  fn fill_if(&mut self, e: &mut Expression, ifbody: &mut Vec<Box<Statement>>, elsebody: &mut Vec<Box<Statement>>){
    assert_eq!(self.get_expr_type(e).unwrap(), Type::Primitive(PrimitiveType::Bool));
    //TODO: Scopes: FIXME:
    for s in ifbody{
      self.fill_stmt(s);
    }
    for s in elsebody{
      self.fill_stmt(s);
    }
  }

  fn fill_ass(&mut self, name: &mut String, mut ty: &mut RefCell<Type>, e: &mut Expression){
    let t1 = self.get_expr_type(e).expect(&format!("Could not determine type of expression: '{:?}'", e));
    if *ty.borrow() == Type::Unknown{
      replace_type(ty, t1.clone());
      println!("Ass: {:?}\n", ty);
      self.variables.insert(name.clone(), t1);
      return;
    }

    if *ty.borrow() != t1{
      panic!("Cannot assign '{:?}' to variable '{:?}', which has type '{:?}'", name, e, ty);
    }
    self.variables.insert(name.clone(), t1);
  }

  fn check_call(&self, fc: &mut FunctionCall) -> Option<Type>{
    let name = fc.name.clone();
    let ft = self.functions.get(&name).expect(&format!("Could not find function: '{:?}'", name));

    if fc.args.len() != ft.params.len(){
      panic!("Expected '{:?}' arguements but found '{:?}' at function call: '{:?}'", ft.params.len(), fc.args.len(), name);
    }

    for (i, e) in fc.args.iter_mut().enumerate(){
      let et = self.get_expr_type(e).expect(&format!("Could not determine type of expression: '{:?}'", e));
      println!("{:?}\n\n", self.tree);
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

  fn check_return(&self, expr: &mut Option<Expression>){
    let et = 
      if let Some(expr) = expr{
        Some(self.get_expr_type(expr).expect(&format!("Could not determine type of expression: '{:?}'", expr)))
      }
      else{
        None
      };
    //void returns
    if et != self.current_ret{
      panic!("Expected return type of '{:?}', but found '{:?}' instead.", self.current_ret, et);
    }
  }

  fn get_expr_type(&self, e: &mut Expression) -> Result<Type, ()>{
		match e{
			Expression::Variable(name, t) => {
        let mut t1 = self.variables.get(name)
          .expect(
            &format!("Variable '{:?}' not found.", name)).clone();
        if *t == Type::Unknown{
          println!("Here {:?}:\n{:?}",name, t1);
          *t = t1.clone();
          Ok(t1)
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
			Expression::UnaryExpr(_, ex, ty) => {
        let et = self.get_expr_type(ex).unwrap();
        replace_type(ty, et.clone());
				Ok(et)
			},
			Expression::BinExpr(_, lhs, rhs, ty) => {
				let lt = self.get_expr_type(lhs).unwrap();
        let rt = self.get_expr_type(rhs).unwrap();
				assert_eq!(lt, rt);

        if *ty.borrow() != Type::Unknown{
          return Ok(ty.borrow().clone());
        }
        
        replace_type(ty, lt);
        Ok(rt)
			}
		}
  }


}

fn replace_type(refcell: &mut RefCell<Type>, t: Type){
  refcell.replace(t);
  // *refcell = RefCell::new(t);
}