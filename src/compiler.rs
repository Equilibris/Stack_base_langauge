use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{
    BasicMetadataValueEnum, BasicValue, FloatValue, FunctionValue, IntValue, PointerValue,
};
use inkwell::{FloatPredicate, OptimizationLevel};

use crate::namemap::NameMap;

struct Compiler<'ctx, 'a> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,

    pub name_map: NameMap,
    pub name_exec_map: HashMap<String, FunctionValue<'ctx>>,
    // pub main_stack_ptr:
}

impl<'ctx, 'a> Compiler<'ctx, 'a> {
    pub fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
        module: &'a Module<'ctx>,

        name_map: NameMap,
    ) -> Self {
        let size = name_map.len();
        let mut stack_ptr = builder;

        Self {
            context,
            builder,
            fpm,
            module,

            name_map,
            name_exec_map: HashMap::with_capacity(size),
        }
    }
}
