use crate::r#type::Type;

#[derive(Debug)]
pub enum Stmt{
  Assign{
    name: String,
    val: Expr,
    type_name: Option<String>,
    r#type: Option<Type>
  },
  Return{
    expr: Option<Expr>
  },
  If{
    cond: Expr,
    trueBranch: Vec<Stmt>,
    falseBranch: Option<Vec<Stmt>>,
  }
}

#[derive(Debug)]
pub enum Expr{
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
  Div(Box<Expr>, Box<Expr>),

  EqualTo(Box<Expr>, Box<Expr>),
  NotEqualTo(Box<Expr>, Box<Expr>),
  LessThan(Box<Expr>, Box<Expr>),
  LessThanEqualTo(Box<Expr>, Box<Expr>),
  GreaterThan(Box<Expr>, Box<Expr>),
  GreaterThanEqualTo(Box<Expr>, Box<Expr>),

  Negate(Box<Expr>),
  
  IntLiteral(i64),
  FloatLiteral(f64),
  StringLiteral(String),
  Variable(String)
}
