use crate::{AstAny, lexer::LexToken, AstResult};

pub trait Handler {
    #[inline]
    fn on_read(&mut self, _token: &mut LexToken) -> AstResult<AstAny> {
        Ok(AstAny::Unsport)
    }
}

pub struct DefaultHandler;

impl Handler for DefaultHandler {
    fn on_read(&mut self, _token: &mut LexToken) -> AstResult<AstAny> {

        Ok(AstAny::Unsport)
    }
}