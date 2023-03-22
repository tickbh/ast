use crate::{LangAny, lexer::LexToken};

pub trait Handler {
    #[inline]
    fn on_read(&mut self, _token: &mut LexToken) -> LangAny {
        LangAny::Unsport
    }
}

pub struct DefaultHandler;

impl Handler for DefaultHandler {
    fn on_read(&mut self, _token: &mut LexToken) -> LangAny {

        LangAny::Unsport
    }
}