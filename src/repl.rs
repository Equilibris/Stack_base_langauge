use inkwell::{context::Context, passes::PassManager, values::FunctionValue};

use std::io::Write;

use crate::tokenizer::{tokenizer, Token};

pub fn repl() {
    let debug_lexer_out = false;
    let debug_ast_out = false;
    let debug_comp_out = false;

    let ccontext = Context::create();
    let module = ccontext.create_module("repl");
    let builder = ccontext.create_builder();

    let cmanager: PassManager<FunctionValue> = PassManager::create(&module);

    cmanager.add_instruction_combining_pass();
    cmanager.add_reassociate_pass();
    cmanager.add_gvn_pass();
    cmanager.add_cfg_simplification_pass();
    cmanager.add_basic_alias_analysis_pass();
    cmanager.add_promote_memory_to_register_pass();
    cmanager.add_instruction_combining_pass();
    cmanager.add_reassociate_pass();

    cmanager.initialize();

    let mut accumulator: Vec<Token> = Vec::new();

    loop {
        println!();
        print!("sbl > ");
        std::io::stdout().flush().unwrap();

        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();

        match tokenizer(s) {
            Ok(res) => (),
            Err(err) => {
                println!("A tokenizer error occurred:\n{}\nThis means the expression is not added to the token buffer", err);
                continue;
            }
        };
    }
}
