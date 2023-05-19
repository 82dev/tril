use std::{path::PathBuf, collections::HashMap};

use inkwell::{context::Context, module::Module, builder::Builder, values::{FloatValue, BasicMetadataValueEnum, PointerValue, BasicValue}, types::{FloatType, PointerType}};

use crate::nodes::{Stmt, Expr, UnOp, BinOp};

pub struct CodeGenerator<'ctx>{
	pub context: &'ctx Context,
	pub module: Module<'ctx>,
	pub builder: Builder<'ctx>,
	pub nodes: Vec<Stmt>,
	variables: HashMap<String, PointerValue<'ctx>>,
	f32_type: FloatType<'ctx>,
}

impl<'ctx> CodeGenerator<'ctx>{
	pub fn new(context: &'ctx Context, module: Module<'ctx>, builder: Builder<'ctx>, nodes: Vec<Stmt>) -> CodeGenerator<'ctx>{
		CodeGenerator{
			context,
			module,
			builder,
			nodes,
			variables: HashMap::new(),
			f32_type: context.f32_type(),
		}
	}

	pub fn generate(mut self, path: &PathBuf){
		let fprintf_t = self.f32_type.fn_type(&vec![self.f32_type.into()], false);
		let fprintf = self.module.add_function("print_ascii", fprintf_t, None);

		for stmt in self.nodes.clone().iter(){
			self.gen_stmt(&stmt);
		}
		
		println!("{}", path.display());
		self.module.print_to_file(path);
	}

	fn gen_stmt(&mut self, stmt: &Stmt){
		match stmt{
			Stmt::Assignment(var, expr) => self.gen_ass(var.0.clone(), expr.clone()),
			// Stmt::FnDef(name, params, stmts) => self.gen_fn(name.to_string(), params.clone(), stmts.clone()),
			// Stmt::FnCall(call) => {self.gen_call(call.0.clone(), call.1.clone());},
			Stmt::Return(expr) => {self.gen_ret(expr.clone())},
			_ => ()
		}
	}

	fn gen_ret(&mut self, expr: Expr){
		let e = self.gen_expr(expr.clone());
		self.builder
			.build_return(
				Some(&e)
			);
	}

	fn gen_fn(
		&mut self,
		name: String,
		params: Vec<String>,
		stmts: Vec<Stmt>)
	{	
		let fn_type = self.f32_type.fn_type(
			&vec![self.f32_type.into(); params.len()],
			false
		);
		
		let func = self.module.add_function(name.as_str(), fn_type, None);
		
		//https://github.com/Narukara/Kaleidoscope-Rust/blob/main/src/parser/mod.rs
		//ty <3
		let args: Vec<FloatValue> = func
			.get_param_iter()
			.map(|p| p.into_float_value())
			.collect();

		let bb = self.context.append_basic_block(func, "entry");
		self.builder.position_at_end(bb);

		self.variables.clear();
		for (i, p) in args.iter().enumerate(){
			p.set_name(params[i].as_str());

			let ptr = self.builder.build_alloca(self.f32_type, params[i].as_str());
			self.builder.build_store(ptr, args[i]);
			self.variables.insert(params[i].clone(), ptr);
		}

		for stmt in stmts{
			self.gen_stmt(&stmt);
		}
	}

	fn gen_call(&mut self, name: String, args: Vec<Expr>) -> FloatValue<'ctx>{
		let func = self.module
			.get_function(name.as_str())
			.unwrap();

		if func.count_params() as usize != args.len(){
			panic!("Number of parameters does not match function! {}", name);
		}

		let args: Vec<BasicMetadataValueEnum> = args
			.iter()
			.map(|a| self.gen_expr(a.clone()).into())
			.collect();

		self.builder
			.build_call(func, &args, name.as_str())
			.try_as_basic_value()
			.left()
			.unwrap()
			.into_float_value()
	}

	fn gen_ass(&mut self, name: String, expr: Expr){
		let v = self.builder.build_alloca(self.f32_type, name.as_str());
		let e = self.gen_expr(expr);
		self.builder
			.build_store(v, e);

		self.variables.insert(name, v);
	}

	fn gen_expr(&mut self, expr: Expr) -> FloatValue<'ctx>{
		match expr{
			Expr::Number(val) => {self.f32_type.const_float(val.into())},
			Expr::BinaryExpr(lhs, bop, rhs) => {
				let l = self.gen_expr(lhs.into());
				let r = self.gen_expr(rhs.into());

				match bop{
					BinOp::Plus => {self.builder.build_float_add(l, r, "addtmp")},
					BinOp::Minus => {self.builder.build_float_sub(l, r, "subtmp")},
					BinOp::Asterisk => {self.builder.build_float_mul(l, r, "multmp")},
					BinOp::FSlash => {self.builder.build_float_div(l, r, "divtmp")},
				}
			},
			Expr::UnaryExpr(uop, rhs) => {
				match uop{
					UnOp::Minus => {
						let r = self.gen_expr(rhs.into());
						self.builder.build_float_neg(r, "negtmp")
					}
				}
			},
			Expr::Var(var) => {
				//TODO: idk tf i'm doing
				// self.module.get_global(name.as_str()).unwrap().get_initializer().unwrap().into_float_value()
				match self.variables.get(&var.0){
					Some(n) => self.builder.build_load(self.f32_type, n.clone(), var.0.as_str()).into_float_value(),
					None => panic!("Could not find variable '{}'", var.0),
				}
			},
			Expr::FnCall(call) => {
				self.gen_call(call.0, call.1)
			}
			_ => panic!(),
		}
	}
}
