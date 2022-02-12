mod ast;
mod numeric_litteral;
mod tokenizer;

fn main() {
    println!(
        "{:?}",
        tokenizer::tokenizer("@main { \"Hello world!\" . };".to_string())
    );
}
