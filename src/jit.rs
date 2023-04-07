use cranelift::prelude::*;
use cranelift_jit::{JITModule, JITBuilder};
use cranelift_module::DataContext;

struct JIT {
    builder_context: FunctionBuilderContext, 
    ctx: codegen::Context,
    data_ctx: DataContext,
    module: JITModule,
}
