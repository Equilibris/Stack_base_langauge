mod ast;
mod compiler;
mod namemap;
mod numeric_litteral;
mod repl;
mod tokenizer;
// mod colidescope;

fn main() {
    repl::repl();
}

// fn main() -> anyhow::Result<()> {
//     let program = "@main { \"Hello world!\" . } ; this be hello world".to_string();
//     let program = tokenizer::tokenizer(program)?;

//     println!("{:?}", program);

//     let program = ast::build_tree(program)?;

//     println!("{:?}", program);

//     Ok(())
// }
