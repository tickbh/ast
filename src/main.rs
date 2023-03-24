extern crate lang_ast;

use std::str::FromStr;
use lang_ast::{Lexer, Handler, AstAny, LexToken, AstResult};
use regex::Regex;

struct CalcHandler;
impl Handler for CalcHandler {
    fn on_read(&mut self, _token: &mut LexToken) -> AstResult<AstAny> {
        println!("token = {:?}", _token);
        match _token.ty {
            "num" => {
                Ok(AstAny::I64(i64::from_str(_token.get_value())?))
            },
            _ => {
                Ok(AstAny::Unsport)
            }
        }
    }
}
fn main() {
// t_STRING = r'\"([^\\\n]|(\\.))*?\"'
// # Identifiers
// t_ID = r'[A-Za-z_][A-Za-z0-9_]*'

// # Integer literal
// t_INTEGER = r'\d+([uU]|[lL]|[uU][lL]|[lL][uU])?'

// # Floating literal
// t_FLOAT = r'((\d+)(\.\d+)(e(\+|-)?(\d+))? | SB(\d+)e(\+|-)?(\d+))([lL]|[fF])?'

// # String literal
// t_STRING = r'\"([^\\\n]|(\\.))*?\"'
    let value = "right11=2211;\n
    
    aaass=222uiii".to_string();
    let value = "1 + 2*-3".to_string();
    // let value = "function xx() end".to_string();
    let xx = Regex::new(r"[A-Za-z_][A-Za-z0-9_]*").unwrap();
    println!("======={:?}", xx.shortest_match_at(&value, 5));

    let mut lex = Lexer::new(value, CalcHandler{});
    lex.add_hash_match("id", "function", "end");

    lex.add_regex("id", Regex::new(r"[A-Za-z_][A-Za-z0-9_]*").unwrap());

    lex.add_regex("equal", Regex::new(r"=").unwrap());
    
    lex.add_regex("end", Regex::new(r";").unwrap());
    lex.add_regex("line", Regex::new(r"\n").unwrap());
    lex.add_regex("num", Regex::new(r"\d+([uU]|[lL]|[uU][lL]|[lL][uU])?").unwrap());

    let result = lex.eval();
    println!("Hello, world! {:?}", result);
}
