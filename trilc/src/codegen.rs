use std::{path::PathBuf, collections::HashMap, unreachable, todo, matches, rc::Rc, cell::RefCell};
use either::Either;

use inkwell::{context::Context, module::Module, builder::Builder, values::{FloatValue, BasicMetadataValueEnum, PointerValue, BasicValue, BasicValueEnum, InstructionValue, ArrayValue, IntValue}, types::{FloatType, PointerType, BasicTypeEnum, BasicType, BasicMetadataTypeEnum }, AddressSpace, IntPredicate, FloatPredicate};

use crate::{nodes::{UnOp, BinOp, TopLevel, Statement, Expression, Literal, FunctionCall}, types::{Type, PrimitiveType, FunctionType}};

pub struct CodeGenerator<'ctx>{
	pub context: &'ctx Context,
	pub module: Module<'ctx>,
	pub builder: Builder<'ctx>,
	pub nodes: Vec<TopLevel>,
	variables: HashMap<String, PointerValue<'ctx>>,
	functions: HashMap<String, FunctionType>,
	zero: IntValue<'ctx>,
}

impl<'ctx> CodeGenerator<'ctx>{
	pub fn new(context: &'ctx Context, module: Module<'ctx>, builder: Builder<'ctx>, nodes: Vec<TopLevel>, functions: HashMap<String, FunctionType>) -> CodeGenerator<'ctx>{
		CodeGenerator{
			context,
			module,
			builder,
			nodes,
			variables: HashMap::new(),
			functions,
			zero: context.i32_type().const_int(0, false),
		}
	}

	pub fn generate(mut self, path: &PathBuf) -> Vec<TopLevel>{
		println!("{}", path.display());
		for top in self.nodes.to_owned(){
			self.gen_top(top);
		}
		self.module.print_to_file(path).unwrap();

		self.nodes
	}

	fn gen_top(&mut self, top: TopLevel){
		match top{
			TopLevel::FnDecl(name, ftype, params, body) => self.gen_fn_decl(name, ftype, params, body),
			TopLevel::Extern(name, ftype) => self.gen_extern(name, ftype),
			TopLevel::StructDecl(_) => todo!(),
		}
	}

	fn gen_statement(&mut self, stmt: Statement){
		match stmt{
			Statement::While(expr, body) => self.gen_while(expr, body),
			Statement::Assignment(name, ty, expr) => self.gen_ass(name, ty, expr),
			Statement::Mutate(name, expr) => self.gen_mut(name, expr),
			Statement::FnCall(fcall) => {self.gen_call(fcall);},
			Statement::Return(expr) => self.gen_return(expr),
			Statement::If(expr, ifbody, elsebody) => self.gen_if(expr, ifbody, elsebody),
		}
	}

	fn gen_while(&mut self, expr: Expression, body: Vec<Box<Statement>>){
		let func = self.builder.get_insert_block().unwrap()
			.get_parent().unwrap();
	
		let whilecond = self.context.append_basic_block(func, "whilecond");
		let whileloop = self.context.append_basic_block(func, "whileloop");
		let afterwhile = self.context.append_basic_block(func, "afterwhile");

		self.builder.build_unconditional_branch(whilecond);

		self.builder.position_at_end(whilecond);
		//This should be a bool so we can safely? into_int_value();
		let e = self.gen_expression(expr).into_int_value();
		let cond = self.builder
			.build_int_compare(IntPredicate::NE, e, self.context.bool_type().const_int(0, false), "whilecond");
		self.builder.build_conditional_branch(cond, whileloop, afterwhile);

		self.builder.position_at_end(whileloop);
		for s in body{
			self.gen_statement(*s);
		}
		self.builder.build_unconditional_branch(whilecond);

		self.builder.position_at_end(afterwhile);
	}

	fn gen_if(&mut self, expr: Expression, ifbody: Vec<Box<Statement>>, elsebody: Vec<Box<Statement>>){
		let bb = self.builder.get_insert_block().unwrap();
		let func = bb.get_parent().unwrap();
		//This should be a bool so we can safely? into_int_value();
		let e = self.gen_expression(expr).into_int_value();
		let cond = self.builder
			.build_int_compare(IntPredicate::NE, e, self.context.bool_type().const_int(0, false), "ifcond");

		let thenbb = self.context.append_basic_block(func, "then");
		let elsebb = self.context.append_basic_block(func, "else"); //TODO: Else blocks
		let mergebb = self.context.append_basic_block(func, "merge");

		self.builder.build_conditional_branch(cond, thenbb, elsebb);

		self.builder.position_at_end(thenbb);
		for s in ifbody{
			self.gen_statement(*s);
		}
		self.builder.build_unconditional_branch(mergebb);

		
		self.builder.position_at_end(elsebb);
		for s in elsebody{
			self.gen_statement(*s);
		}
		self.builder.build_unconditional_branch(mergebb);

		self.builder.position_at_end(mergebb);
	}

	fn gen_return(&mut self, expr: Option<Expression>){
		if let Some(expr) = expr{
			let e = self.gen_expression(expr);
			self.builder
				.build_return(Some(&e));
		}
		else{
			self.builder
				.build_return(None);
		}
	}

	fn gen_mut(&mut self, name: String, expr: Expression){
		let ptr = self.variables.get(&name).unwrap().clone();
		let e = self.gen_expression(expr);
		self.builder
			.build_store(ptr, e);
	}

	fn gen_ass(
		&mut self,
		name: String,
		ty: RefCell<Type>,
		expr: Expression,
	){
		let ty = ty.into_inner();
		let ptr = self.builder.build_alloca(self.get_type(ty), name.as_str());
		let e = self.gen_expression(expr);
		self.builder.build_store(ptr, e);
		self.variables.insert(name, ptr);
	}

	fn gen_fn_decl(
		&mut self,
		name: String,
		ftype: FunctionType,
		params: Vec<String>,
		body: Vec<Statement>)
	{
		let ft = self.get_fn_type(ftype);
		let func = self.module.add_function(name.as_str(), ft, None);

		let bb = self.context.append_basic_block(func, "entry");
		self.builder.position_at_end(bb);

		self.variables.clear();
		for (i, p) in func.get_params().iter().enumerate(){
			let pname = params[i].as_str();
			p.set_name(&pname);

			let ptr = self.builder.build_alloca(p.get_type(), &pname);
			self.builder.build_store(ptr, *p);
			self.variables.insert(pname.to_string(), ptr);
		}

		for stmt in body{
			self.gen_statement(stmt);
		}
	}

	fn gen_call(&self, fcall: FunctionCall) -> Either<BasicValueEnum<'ctx>, InstructionValue<'ctx>>{
		let func = self.module.get_function(&fcall.name).unwrap();
		assert_eq!(func.count_params() as usize, fcall.args.len());

		let args: Vec<BasicMetadataValueEnum> = fcall.args.iter()
			.map(|a| self.gen_expression(a.clone()).into()).collect();
		
		self.builder
			.build_call(func, &args, &fcall.name)
			.try_as_basic_value()
	}
	
	fn gen_extern(
		&mut self,
		name: String, 
		ftype: FunctionType,
	)
	{
		let ft = self.get_fn_type(ftype);
		let func = self.module.add_function(name.as_str(), ft, None);
	}

	fn gen_expression(&self, expr: Expression) -> BasicValueEnum<'ctx>{
		match expr{
			Expression::Literal(lit) => {
				match lit{
					Literal::Int(i) => self.context.i32_type().const_int(i as u64, false).as_basic_value_enum(),
					Literal::Float(f) => self.context.f32_type().const_float(f as f64).as_basic_value_enum(),
					Literal::Bool(b) => self.context.bool_type().const_int(if b {1} else {0}, false).as_basic_value_enum(),
					Literal::String(s) => {
						self.builder.build_global_string_ptr(s.as_str(), "str").as_pointer_value().as_basic_value_enum()
					},
				}
			},

			Expression::Variable(name, ty) => {
				self.builder.build_load(self.get_type(ty), *self.variables.get(&name).unwrap(), &name)
			},

			Expression::FnCall(fcall) => {
				self.gen_call(fcall).left().unwrap()
			},

			Expression::UnaryExpr(unop, e, ty) => {
				let ty = ty.into_inner();
				if let Type::Primitive(ty) = ty{
					let rhs = self.gen_expression(e.into());
					match unop{
						UnOp::ArithmeticNeg => {
							match ty{
								PrimitiveType::Int => {
									self.builder.build_int_neg(rhs.into_int_value(), "ineg").as_basic_value_enum()
								},
								PrimitiveType::Float => {
									self.builder.build_float_neg(rhs.into_float_value(), "fneg").as_basic_value_enum()
								},

								_ => panic!()
							}
						}
					}
				}
				else{
					unreachable!()
				}
			}

			//To whoever is reading this, I'm sorry
			Expression::BinExpr(
				bop,
				lhs,
				rhs,
				ty,
			) => {
				let r#type = self.get_expr_type(*lhs.clone());
				let lhs = self.gen_expression(lhs.into());
				let rhs = self.gen_expression(rhs.into());

				if let Type::Primitive(r#type) = r#type{
					match r#type{
						PrimitiveType::Int => {
							let lhs = lhs.into_int_value();
							let rhs = rhs.into_int_value();
							match bop{
								BinOp::Add => self.builder.build_int_add(lhs, rhs, "iadd").as_basic_value_enum(),
								BinOp::Sub => self.builder.build_int_sub(lhs, rhs, "isub").as_basic_value_enum(),
								BinOp::Mul => self.builder.build_int_mul(lhs, rhs, "imul").as_basic_value_enum(),
								BinOp::Div => self.builder.build_int_signed_div(lhs, rhs, "isdiv").as_basic_value_enum(),
								BinOp::Equal => self.builder.build_int_compare(IntPredicate::EQ, lhs, rhs, "ieq").as_basic_value_enum(),
								BinOp::NEqual => self.builder.build_int_compare(IntPredicate::NE, lhs, rhs, "ine").as_basic_value_enum(),
								BinOp::Lesser => self.builder.build_int_compare(IntPredicate::SLT, lhs, rhs, "ilt").as_basic_value_enum(),
								BinOp::LEq => self.builder.build_int_compare(IntPredicate::SLE, lhs, rhs, "ile").as_basic_value_enum(),
								BinOp::Greater => self.builder.build_int_compare(IntPredicate::SGT, lhs, rhs, "igt").as_basic_value_enum(),
								BinOp::GEq => self.builder.build_int_compare(IntPredicate::SGE, lhs, rhs, "ige").as_basic_value_enum(),
							}
						},
						PrimitiveType::Float => {
							let lhs = lhs.into_float_value();
							let rhs = rhs.into_float_value();
							match bop{
								BinOp::Add => self.builder.build_float_add(lhs, rhs, "fadd").as_basic_value_enum(),
								BinOp::Sub => self.builder.build_float_sub(lhs, rhs, "fsub").as_basic_value_enum(),
								BinOp::Mul => self.builder.build_float_mul(lhs, rhs, "fmul").as_basic_value_enum(),
								BinOp::Div => self.builder.build_float_div(lhs, rhs, "fsdiv").as_basic_value_enum(),
								BinOp::Equal => self.builder.build_float_compare(FloatPredicate::OEQ, lhs, rhs, "feq").as_basic_value_enum(),
								BinOp::NEqual => self.builder.build_float_compare(FloatPredicate::ONE, lhs, rhs, "fne").as_basic_value_enum(),
								BinOp::Lesser => self.builder.build_float_compare(FloatPredicate::OLT, lhs, rhs, "flt").as_basic_value_enum(),
								BinOp::LEq => self.builder.build_float_compare(FloatPredicate::OLE, lhs, rhs, "fle").as_basic_value_enum(),
								BinOp::Greater => self.builder.build_float_compare(FloatPredicate::OGT, lhs, rhs, "fgt").as_basic_value_enum(),
								BinOp::GEq => self.builder.build_float_compare(FloatPredicate::OGE, lhs, rhs, "fge").as_basic_value_enum(),
							}
						},
						PrimitiveType::Bool => todo!(),
						PrimitiveType::String => panic!(),
						PrimitiveType::Array(elems, size) => todo!(),
					}
				}
				else {unreachable!()}
			}
		}
	}

	fn get_type(&self, t: Type) -> BasicTypeEnum<'ctx>{
		match t{
			Type::Primitive(p) => self.get_type_from_primitive(p),
			Type::Func(ft) => {
				self.get_fn_type(ft).ptr_type(AddressSpace::default()).into()
			}
			Type::Struct(_) => todo!(),
			Type::Unknown => unreachable!(),
			Type::Void => unreachable!(),
		}
	}

	fn get_expr_type(&self, e: Expression) -> Type{
		match e{
			Expression::Variable(_, t) => t,
			Expression::Literal(lit) => match lit{
				Literal::Int(_) => Type::Primitive(PrimitiveType::Int),
				Literal::Float(_) => Type::Primitive(PrimitiveType::Float),
				Literal::String(_) => Type::Primitive(PrimitiveType::String),
				Literal::Bool(_) => Type::Primitive(PrimitiveType::Bool),
			},
			Expression::FnCall(fcall) => {
				//TODO: WTF FIXME:
				self.functions.get(&fcall.name).unwrap().ret.clone().expect("Function {fcall.name} returns VOID!").as_ref().clone()
			},
			Expression::UnaryExpr(_, ex, ty) => {
				ty.into_inner()
			},
			Expression::BinExpr(_, lhs, rhs, mut ty) => {
				ty.into_inner()
			}
		}
	}

	fn get_fn_type(&self, ft: FunctionType) -> inkwell::types::FunctionType<'ctx>{
		let params: &Vec<BasicMetadataTypeEnum> =
			&ft.params.iter().map(
				|p| {
					self.get_type(*p.clone()).into()
				}
			).collect();
		
		if let Some(ret) = ft.ret{
			self.get_type(ret.into()).fn_type(params, false)
		}
		else{
			self.context.void_type().fn_type(params, false)
		}
	}

	fn get_type_from_primitive(&self, t: PrimitiveType) -> BasicTypeEnum<'ctx>{
		match t{
			PrimitiveType::Float => self.context.f32_type().into(),
			PrimitiveType::String => self.context
				.i8_type()
				.ptr_type(AddressSpace::default())
				.into(),
			PrimitiveType::Int => self.context.i32_type().into(),
			PrimitiveType::Bool => self.context.bool_type().into(),
			PrimitiveType::Array(ty, size) => self.get_type(*ty).array_type(size.unwrap()).into()
		}
	}
}
