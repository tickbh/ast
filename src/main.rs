extern crate lang_ast;

use lang_ast::Lexer;
fn main() {
    let lex = Lexer::new("1+1".to_string());
    println!("Hello, world!");
}
