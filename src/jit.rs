use std::collections::HashMap;

use crate::parser;
use crate::BinaryOp;
use crate::Expr;
use crate::Result;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::Linkage;
use cranelift_module::Module;

pub struct JIT {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    module: JITModule,
}

impl Default for JIT {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        let module = JITModule::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            module,
        }
    }
}

impl JIT {
    pub fn compile(&mut self, source: &str) -> Result<*const u8> {
        let ast = parser::file(source)?;

        for node in ast {
            match node {
                Expr::Function(name, params, stmts) => {
                    self.translate(params, stmts, "none")?;

                    let id = self
                        .module
                        .declare_function(&name, Linkage::Export, &self.ctx.func.signature)
                        .map_err(|e| e.to_string())?;

                    self.module
                        .define_function(id, &mut self.ctx)
                        .map_err(|e| e.to_string())?;

                    self.module.clear_context(&mut self.ctx);

                    self.module.finalize_definitions().unwrap();

                    let code = self.module.get_finalized_function(id);

                    return Ok(code);
                }
                _ => todo!("Implement all branches of compile"),
            }
        }

        Err("Could not compile".into())
    }

    fn translate(&mut self, params: Vec<String>, stmts: Vec<Expr>, the_return: &str) -> Result<()> {
        let int = self.module.target_config().pointer_type();

        for _p in &params {
            self.ctx.func.signature.params.push(AbiParam::new(int));
        }

        self.ctx.func.signature.returns.push(AbiParam::new(int));

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        let entry_block = builder.create_block();

        builder.append_block_params_for_function_params(entry_block);

        builder.switch_to_block(entry_block);

        builder.seal_block(entry_block);

        let variables =
            declare_variables(int, &mut builder, &params, the_return, &stmts, entry_block);

        let mut translator = FunctionTranslator {
            int,
            builder,
            variables,
            module: &mut self.module,
        };

        for expr in stmts {
            translator.translate_expr(expr);
        }

        let return_variable = translator.variables.get(the_return).unwrap();
        let return_value = translator.builder.use_var(*return_variable);

        translator.builder.ins().return_(&[return_value]);
        translator.builder.finalize();
        Ok(())
    }
}

struct FunctionTranslator<'a> {
    int: types::Type,
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    module: &'a mut JITModule,
}

impl<'a> FunctionTranslator<'a> {
    pub fn translate_expr(&mut self, expr: Expr) -> Value {
        use Expr::*;

        match expr {
            Literal(val) => {
                let imm: i32 = val.parse().unwrap();
                self.builder.ins().iconst(self.int, i64::from(imm))
            }
            Operation(lhs, rhs, op) => {
                let lhs = self.translate_expr(*lhs);
                let rhs = self.translate_expr(*rhs);
                self.tranalate_operation(lhs, rhs, op)
            }
            Assign(name, expr) => self.translate_assign(name, *expr),
            Identifier(name) => {
                let variable = self.variables.get(&name).expect("variable not defined");
                self.builder.use_var(*variable)
            }
            GlobalDataAddr(name) => self.translate_global_data_addr(name),
            _ => todo!("Implement all branches"),
        }
    }

    fn translate_assign(&mut self, name: String, expr: Expr) -> Value {
        let new_value = self.translate_expr(expr);
        let variable = self.variables.get(&name).unwrap();
        self.builder.def_var(*variable, new_value);
        new_value
    }

    fn tranalate_operation(&mut self, lhs: Value, rhs: Value, op: BinaryOp) -> Value {
        use BinaryOp::*;
        match op {
            Add => self.builder.ins().iadd(lhs, rhs),
            Sub => self.builder.ins().isub(lhs, rhs),
            Mul => self.builder.ins().imul(lhs, rhs),
            Div => self.builder.ins().udiv(lhs, rhs),
        }
    }

    fn translate_global_data_addr(&mut self, name: String) -> Value {
        let sym = self
            .module
            .declare_data(&name, Linkage::Export, true, false)
            .expect("problem declaring data");
        let local_id = self.module.declare_data_in_func(sym, self.builder.func);

        let pointer = self.module.target_config().pointer_type();
        self.builder.ins().symbol_value(pointer, local_id)
    }
}

fn declare_variable(
    int: types::Type,
    builder: &mut FunctionBuilder,
    variables: &mut HashMap<String, Variable>,
    index: &mut usize,
    name: &str,
) -> Variable {
    let var = Variable::new(*index);

    if !variables.contains_key(name) {
        variables.insert(name.into(), var);
        builder.declare_var(var, int);
        *index += 1;
    }

    var
}

fn declare_variable_in_stmt(
    int: types::Type,
    builder: &mut FunctionBuilder,
    variables: &mut HashMap<String, Variable>,
    index: &mut usize,
    expr: &Expr,
) {
    match *expr {
        Expr::Assign(ref name, _) => {
            declare_variable(int, builder, variables, index, name);
        }
        _ => {}
    }
}

fn declare_variables(
    int: types::Type,
    builder: &mut FunctionBuilder,
    params: &[String],
    the_return: &str,
    stmts: &[Expr],
    entry_block: Block,
) -> HashMap<String, Variable> {
    let mut variables = HashMap::new();
    let mut index = 0;

    for (i, name) in params.iter().enumerate() {
        let val = builder.block_params(entry_block)[i];
        let var = declare_variable(int, builder, &mut variables, &mut index, name);
        builder.def_var(var, val);
    }

    let zero = builder.ins().iconst(int, 0);

    let the_return = declare_variable(int, builder, &mut variables, &mut index, the_return);
    builder.def_var(the_return, zero);

    for expr in stmts {
        declare_variable_in_stmt(int, builder, &mut variables, &mut index, expr);
    }

    variables
}
