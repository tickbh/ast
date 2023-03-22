use std::{ops::BitAnd, sync::Arc, collections::HashMap};
use regex::Regex;
use std::fmt::Debug;

use crate::{Handler, LangAny};

#[derive(Clone)]
pub struct LexToken {
    pub ty: &'static str,
    pub data: Arc<String>,
    pub lineno: usize,
    pub start: usize,
    pub end: usize,
    pub subs: Vec<LexToken>,
    pub value: LangAny,
}

impl Debug for LexToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.data.get(self.start..self.end).unwrap();
        f.debug_struct("LexToken").field("ty", &self.ty).field("value", &val).field("lineno", &self.lineno).field("start", &self.start).field("end", &self.end).field("subs", &self.subs).field("value", &self.value).finish()
    }
}

impl LexToken {
    pub fn get_value(&self) -> &str {
        self.data.get(self.start..self.end).unwrap()
    }

    pub fn clone_base_token(&self) -> LexToken {
        LexToken { ty: self.ty, data: self.data.clone(), lineno: self.lineno, start: self.start, end: self.end, subs: vec![], value: LangAny::Unknow }
    }
}

#[derive(Clone, Debug)]
pub struct LexGroupToken {
    pub tokens: Vec<LexGroupToken>,
}

#[derive(Clone, Debug)]
pub enum GroupOrToken {
    Group(Vec<LexToken>),
    Token(LexToken),
}

impl GroupOrToken {
    pub fn group() -> GroupOrToken {
        GroupOrToken::Group(vec![])
    }

    pub fn group_by_token(token: LexToken) -> GroupOrToken {
        GroupOrToken::Group(vec![token])
    }
}

#[derive(Clone, Debug)]
pub struct LexRegex {
    pub re: Regex,
    pub ty: &'static str,
}

#[derive(Clone, Debug)]
pub struct Lexer<H>
where H: Handler {
    pub res: Vec<LexRegex>,
    pub data: Arc<String>,
    pub tokenstack: Vec<LexToken>,
    pub wait_token: Vec<LexToken>,
    pub pos: usize,
    pub len: usize,
    pub handler: H,
    pub ignore: &'static str,
    pub literals: &'static str,
    pub hash_matchs: HashMap<(&'static str, &'static str), &'static str>,
}

// impl Default for Lexer<DefaultHandler> {
//     fn default() -> Self { 
//         Lexer {
//             res: vec![],
//             data: Arc::new(String::new()),
//             tokenstack: vec![],
//             wait_token: vec![],
//             pos: 0,
//             len: 0,
//             handler: DefaultHandler {},
//             ignore: " \t",
//             literals: "+-*/%^<>=!?()[]{}.,;:",
//             hash_matchs: HashMap::from([
//                 (("lit", "("), ")"),
//                 (("lit", "{"), "}"),
//                 (("lit", "["), "]"),
//             ]),
//         }
//      }
// }

impl<H> Lexer<H> where H: Handler {
    pub fn new(data: String, handler: H) -> Lexer<H> {
        Lexer {
            res: vec![],
            data: Arc::new(data),
            tokenstack: vec![],
            wait_token: vec![],
            pos: 0,
            len: 0,
            handler: handler,
            ignore: " \t",
            literals: "+-*/%^<>=!?()[]{}.,;:",
            hash_matchs: HashMap::from([
                (("lit", "("), ")"),
                (("lit", "{"), "}"),
                (("lit", "["), "]"),
            ]),
        }

        // Lexer {  len: data.len(), data: Arc::new(data), handler, ..Default::default() }
    }

    pub fn add_regex(&mut self, ty: &'static str, re: Regex) {
        let reg = LexRegex {
            ty, re
        };
        self.res.push(reg);
    }

    pub fn add_hash_match(&mut self, ty: &'static str, start: &'static str, end: &'static str, ) {
        self.hash_matchs.insert((ty, start), end);
    }

    pub fn get_next_pos(&self, ori: usize) -> Option<usize> {
        let bytes = self.data.as_bytes();
        if ori >= bytes.len() {
            return None;
        }
        let mut byte = bytes[ori];
        let mut byte_len = 0;
        loop {
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

    pub fn get_now_lineno(&self, pos: usize) -> usize {
        self.data[0..pos].matches("\n").count() + 1
    }

    pub fn get_token(&mut self) -> Option<LexToken> {
        let mut ori = self.pos;
        loop {
            let pos = self.get_next_pos(ori);
            println!("ori = {} pos = {:?}", ori, pos);
            if pos.is_none() {
                return None;
            }
            let val = self.data.get(ori .. pos.unwrap()).unwrap();
            if self.ignore.contains(val) {
                self.pos = pos.unwrap();
                ori = pos.unwrap();
                continue;
            }

            if let Some(lpos) = self.literals.find(val) {
                self.pos = pos.unwrap();
                return Some(LexToken {
                    ty: "lit",
                    data: self.data.clone(),
                    lineno: self.get_now_lineno(ori),
                    start: ori,
                    end: pos.unwrap(),
                    subs: vec![],
                    value: LangAny::Unknow,
                })
            }

            for re in &self.res {
                if let Some(p) = re.re.find_at(&self.data, ori) {
                    if p.start() != ori {
                        continue;
                    }
                    self.pos = p.end();
                    return Some(LexToken {
                        ty: re.ty,
                        data: self.data.clone(),
                        lineno: self.get_now_lineno(ori),
                        start: p.start(),
                        end: p.end(),
                        subs: vec![],
                        value: LangAny::Unknow,
                    })
                }
            }
            println!("now data = {:?}", self.data.get(ori .. pos.unwrap()));
            println!("ori = {:?} pos = {:?}", ori, pos);
            ori = pos.unwrap();
        }
    }

    pub fn read_token(&mut self, token: &mut LexToken) {
        token.value = self.handler.on_read(token);
    }

    pub fn parser(&mut self) {
        self.tokenstack = vec![];
        while let Some(mut token) = self.get_token() {
            println!("token = {:?}", self.hash_matchs);

            println!("token = {:?} 11 = {} match = {}", token, token.ty == "id", token.get_value());

            // println!("token = {:?} match = {}", token, token.ty == "id" && self.hash_matchs.contains_key(token.get_value()));
            self.read_token(&mut token);
            if self.hash_matchs.contains_key(&(token.ty, token.get_value())) {
                self.wait_token.push(token.clone_base_token());
            } else {
                if self.wait_token.len() > 0 {
                    let same_type = {
                        let last = self.wait_token.last().unwrap();
                        if last.ty != token.ty {
                            false
                        } else {
                            self.hash_matchs.get(&(last.ty, last.get_value())) == Some(&token.get_value())
                        }
                    };
                    self.tokenstack.last_mut().unwrap().subs.push(token);
                    if same_type {
                        self.wait_token.pop();
                        if self.wait_token.len() > 0 {
                            let last_group = self.tokenstack.pop().unwrap();
                            self.tokenstack.last_mut().unwrap().subs.push(last_group);
                        }
                    }
                    continue;
                }
            }
            
            self.tokenstack.push(token);
        }

        if self.wait_token.len() > 0 {
            println!("error!!!!!!!!!!!!!! = {:?}", self.wait_token);
        }
        println!("self.tokenstack = {:?}", self.tokenstack);
    }
}
