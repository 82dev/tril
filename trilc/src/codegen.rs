use std::{path::PathBuf, collections::HashMap, unreachable, todo, matches};

use inkwell::{context::Context, module::Module, builder::Builder, values::{FloatValue, BasicMetadataValueEnum, PointerValue, BasicValue, BasicValueEnum}, types::{FloatType, PointerType, BasicTypeEnum, BasicType, BasicMetadataTypeEnum }, AddressSpace};

use crate::{nodes::{UnOp, BinOp, TopLevel, Statement, Expression, Literal, FunctionCall}, types::{Type, PrimitiveType, FunctionType}};

pub struct CodeGenerator<'ctx>{
	pub context: &'ctx Context,
	pub module: Module<'ctx>,
	pub builder: Builder<'ctx>,
	pub nodes: Vec<TopLevel>,
	variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> CodeGenerator<'ctx>{
	pub fn new(context: &'ctx Context, module: Module<'ctx>, builder: Builder<'ctx>, nodes: Vec<TopLevel>) -> CodeGenerator<'ctx>{
		CodeGenerator{
			context,
			module,
			builder,
			nodes,
			variables: HashMap::new(),
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
			Statement::Assignment(name, ty, expr) => self.gen_ass(name, ty, expr),
			Statement::FnCall(fcall) => {self.gen_call(fcall);},
			Statement::Return(expr) => self.gen_return(expr),
		}
	}

	fn gen_return(&mut self, expr: Expression){
		let e = self.gen_expression(expr);
		self.builder
			.build_return(Some(&e));
	}

	fn gen_ass(
		&mut self,
		name: String,
		ty: Type,
		expr: Expression,
	){
		let mut ty = ty;
		
		if matches!(ty, Type::Unknown){
			ty = self.get_expr_type(expr.clone());
		}
		
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
			self.builder.build_store(ptr, p.into_float_value());
			self.variables.insert(pname.to_string(), p.into_pointer_value());
		}

		for stmt in body{
			self.gen_statement(stmt);
		}
	}

	fn gen_call(&mut self, fcall: FunctionCall) -> BasicValueEnum<'ctx>{
		let func = self.module.get_function(&fcall.name).unwrap();
		assert_eq!(func.count_params() as usize, fcall.args.len());

		let args: Vec<BasicMetadataValueEnum> = fcall.args.iter()
			.map(|a| self.gen_expression(a.clone()).into()).collect();
		
		self.builder
			.build_call(func, &args, &fcall.name)
			.try_as_basic_value()
			.left()
			.unwrap()
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

	fn gen_expression(&mut self, expr: Expression) -> BasicValueEnum<'ctx>{
		match expr{
			Expression::Literal(lit) => {
				match lit{
					Literal::Int(i) => self.context.i32_type().const_int(i as u64, false).as_basic_value_enum(),
					Literal::Float(f) => self.context.f32_type().const_float(f as f64).as_basic_value_enum(),
					Literal::Bool(b) => self.context.bool_type().const_int(if b {1} else {0}, false).as_basic_value_enum(),
					Literal::String(s) => {
						self.builder.build_global_string_ptr(s.as_str(), "str").as_pointer_value().as_basic_value_enum()
					}
				}
			},

			Expression::Variable(name, ty) => {
				self.builder.build_load(self.get_type(ty), *self.variables.get(&name).unwrap(), &name)
			},

			Expression::FnCall(fcall) => {
				self.gen_call(fcall)
			},

			Expression::UnaryExpr(unop, e, ty) => {
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
				if let Type::Primitive(ty) = ty{
					let lhs = self.gen_expression(lhs.into());
					let rhs = self.gen_expression(rhs.into());
					
					match bop{
						BinOp::Add => {
							match ty {
								PrimitiveType::Int => {
									self.builder
										.build_int_add(
											lhs.into_int_value(),
											rhs.into_int_value(),
											"iadd"
										).as_basic_value_enum()
								},

								PrimitiveType::Float => {
									self.builder
										.build_float_add(
											lhs.into_float_value(),
											rhs.into_float_value(),
											"fadd"
										).as_basic_value_enum()
								},

								PrimitiveType::String | PrimitiveType::Bool => todo!(),
							}
						},
						BinOp::Sub => {
							match ty {
								PrimitiveType::Int => {
									self.builder
										.build_int_sub(
											lhs.into_int_value(),
											rhs.into_int_value(),
											"isub"
										).as_basic_value_enum()
								},

								PrimitiveType::Float => {
									self.builder
										.build_float_sub(
											lhs.into_float_value(),
											rhs.into_float_value(),
											"fsub"
										).as_basic_value_enum()
								},

								_ => panic!()
						}
					},
					
					BinOp::Mul => {
						match ty {
							PrimitiveType::Int => {
								self.builder
									.build_int_mul(
										lhs.into_int_value(),
										rhs.into_int_value(),
										"imul"
									).as_basic_value_enum()
							},

							PrimitiveType::Float => {
								self.builder
									.build_float_add(
										lhs.into_float_value(),
										rhs.into_float_value(),
										"fmul"
									).as_basic_value_enum()
							},

							_ => panic!()
						}
					},

					BinOp::Div => {
						match ty {
							PrimitiveType::Int => {
								self.builder
									.build_int_exact_signed_div(
										lhs.into_int_value(),
										rhs.into_int_value(),
										"idiv"
									).as_basic_value_enum()
							},

							PrimitiveType::Float => {
								self.builder
									.build_float_div(
										lhs.into_float_value(),
										rhs.into_float_value(),
										"fdiv"
									).as_basic_value_enum()
							},

							_ => panic!()
						}
					},
					}
				}
				else{
					unreachable!()
				}
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
				self.module.get_function(&fcall.name).unwrap().get_type();
				todo!()
			},
			Expression::UnaryExpr(_, ex, mut ty) => {
				ty = self.get_expr_type(ex.into());
				ty
			},
			Expression::BinExpr(_, lhs, rhs, mut ty) => {
				let lt = self.get_expr_type(lhs.into());
				assert_eq!(lt, self.get_expr_type(rhs.into()));
				ty = lt;
				ty
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
		}
	}
}
