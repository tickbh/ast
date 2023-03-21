extern crate lang_ast;

use lang_ast::Lexer;
use regex::Regex;
fn main() {
    // let value = "\"1+1\"我是中".to_string();
    // println!("string size = {}", value.len());

    // let value = "right11=22".to_string();

    // let rex = Regex::new(r"[A-Za-z_][A-Za-z0-9_]*").unwrap();
    // match rex.find_at(&value, 0) {
    //     Some(pos) => {
    //         println!("text = {:?}", value.get(0..pos.end()));
    //     }
    //     None => {

    //     }
    // } ;
    // let a = "\"([^\\\n]|(\\.))*?\"";

    // let value = "\"righ\\\"t11=\"22".to_string();
    // let rex = Regex::new(r#""([^\\\n]|(\\.))*?""#).unwrap();
    // match rex.find_at(&value, 0) {
    //     Some(pos) => {
    //         println!("text = {:?}", value.get(0..pos.end()));
    //     }
    //     None => {
    //         println!("not found string");
    //     }
    // } ;

    // let value = r###"""" aaa fdsafdsa  """"###.to_string();
    // let value = "\"\"\"aaa   
    // fdsafdsa  \"\"\"".to_string();
    // println!("aaaavalues = {:?}!!!!!!!", value.as_bytes());
    // let rex = Regex::new(r#""""([^\n]|[^\r]|(\.))*?""""#).unwrap();
    // match rex.find_at(&value, 0) {
    //     Some(pos) => {
    //         println!("text = {:?} end = {}", value.get(0..pos.end()), pos.end());
    //     }
    //     None => {

    //         println!("not found string");
    //     }
    // } ;


    // let value = "22887798989798789789789uL".to_string();
    // let rex = Regex::new(r#"\d+([uU]|[lL]|[uU][lL]|[lL][uU])?"#).unwrap();
    // match rex.find_at(&value, 0) {
    //     Some(pos) => {
    //         println!("text = {:?}", value.get(0..pos.end()));
    //     }
    //     None => {
    //         println!("not found string");
    //     }
    // } ;
    // return;
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
    let value = "1 + (-1+(2+3))*3".to_string();
    let value = "function xx() end".to_string();
    let xx = Regex::new(r"[A-Za-z_][A-Za-z0-9_]*").unwrap();
    println!("======={:?}", xx.shortest_match_at(&value, 5));

    let mut lex = Lexer::new(value);
    lex.add_hash_match("function", "end");
    
    lex.add_regex("id", Regex::new(r"[A-Za-z_][A-Za-z0-9_]*").unwrap());

    lex.add_regex("equal", Regex::new(r"=").unwrap());
    
    lex.add_regex("end", Regex::new(r";").unwrap());
    lex.add_regex("line", Regex::new(r"\n").unwrap());
    lex.add_regex("num", Regex::new(r"\d+([uU]|[lL]|[uU][lL]|[lL][uU])?").unwrap());

    lex.parser();

    println!("Hello, world!");
}
