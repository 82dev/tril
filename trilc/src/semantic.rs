use crate::nodes::Stmt;

pub fn check_node(nodes: &Vec<Stmt>) -> Result<(), String> {
  let var_table: Vec<String> = vec![];
  let func_table: Vec<String> = vec![];

  for stmt in nodes.iter(){
    match stmt{
      
      _ => (),
    }
  }
  
  Ok(())
}