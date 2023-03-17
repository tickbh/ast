use regex::Regex;


pub struct LexToken {
    pub ty: String,
    pub value: String,
    pub lineno: usize,
    pub pos: usize,
}

pub struct Lexer {
    pub res: Vec<Regex>,
    pub data: String,
    pub pos: usize,
    pub len: usize,

    pub ingore: String,
}

impl Default for Lexer {
    fn default() -> Self { 
        Lexer {
            res: vec![],
            data: String::new(),
            pos: 0,
            len: 0,
            ingore: " \t".to_string(),
        }
     }
}

impl Lexer {

    pub fn new(data: String) -> Lexer {
        Lexer {  len: data.len(), data, ..Default::default() }
    }

    pub fn get_token(&mut self) -> () {

    }

    pub fn parser(&mut self) {

    }
}
