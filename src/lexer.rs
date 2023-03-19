use std::{ops::BitAnd, sync::Arc};
use regex::Regex;
use std::fmt::Debug;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct LexToken {
    pub ty: &'static str,
    pub value: Arc<String>,
    pub lineno: usize,
    pub start: usize,
    pub end: usize,
}

impl Debug for LexToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.value.get(self.start..self.end).unwrap();
        f.debug_struct("LexToken").field("ty", &self.ty).field("value", &val).field("lineno", &self.lineno).field("start", &self.start).field("end", &self.end).finish()
    }
}

#[derive(Clone, Debug)]
pub struct LexRegex {
    pub re: Regex,
    pub ty: &'static str,
}

#[derive(Clone, Debug)]
pub struct Lexer {
    pub res: Vec<LexRegex>,
    pub data: Arc<String>,
    pub pos: usize,
    pub len: usize,
    pub ignore: String,
}

impl Default for Lexer {
    fn default() -> Self { 
        Lexer {
            res: vec![],
            data: Arc::new(String::new()),
            pos: 0,
            len: 0,
            ignore: " \t".to_string(),
        }
     }
}

impl Lexer {

    pub fn new(data: String) -> Lexer {
        Lexer {  len: data.len(), data: Arc::new(data), ..Default::default() }
    }

    pub fn add_regex(&mut self, ty: &'static str, re: Regex) {
        let reg = LexRegex {
            ty, re
        };
        self.res.push(reg);
    }

    pub fn get_next_pos(&self, ori: usize) -> Option<usize> {
        let bytes = self.data.as_bytes();
        if ori >= bytes.len() {
            return None;
        }
        let mut byte = bytes[ori];
        let mut byte_len = 0;
        loop {
            println!("byte = {} byte_len = {}", byte, byte_len);
            if byte.bitand(0x80) == 0 {
                println!("byte = {} break", byte);
                break;
            }
            byte_len += 1;
            byte = byte.checked_shl(1).unwrap_or(0);
        }
        byte_len = byte_len.max(1);

        let pos = ori + byte_len;
        if pos > bytes.len() {
            None
        } else {
            Some(pos)
        }
    }

    pub fn get_token(&mut self) -> Option<LexToken> {
        let mut ori = self.pos;
        loop {
            let pos = self.get_next_pos(ori);
            println!("ori = {} pos = {:?}", ori, pos);
            if pos.is_none() {
                return None;
            }
            if self.ignore.contains(self.data.get(ori .. pos.unwrap()).unwrap()) {
                self.pos = pos.unwrap();
                ori = pos.unwrap();
                continue;
            }

            for re in &self.res {

                println!("regex ori = {} pos = {:?}", ori, pos);

                if let Some(p) = re.re.find_at(&self.data, ori) {
                    if p.start() != ori {
                        continue;
                    }
                    self.pos = p.end();
                    return Some(LexToken {
                        ty: re.ty,
                        value: self.data.clone(),
                        lineno: 0,
                        start: p.start(),
                        end: p.end(),
                    })
                }
            }
            println!("now data = {:?}", self.data.get(ori .. pos.unwrap()));
            println!("ori = {:?} pos = {:?}", ori, pos);
            ori = pos.unwrap();
        }
    }

    pub fn parser(&mut self) {

    }
}
