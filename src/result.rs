use std::borrow::Cow;
use std::convert::{From, Into};
use std::error::Error as StdError;
use std::{fmt, num};
use std::result::Result as StdResult;

use crate::LexToken;

pub type AstResult<T> = StdResult<T, AstError>;

/// The type of an error, which may indicate other kinds of errors as the underlying cause.
#[derive(Debug)]
pub enum Kind {
    Internal,
    NoMatchClose(LexToken),
    ParseIntError(num::ParseIntError),
    ParseFloatError(num::ParseFloatError),
    Custom(Box<dyn StdError + Send + Sync>),
}

/// A struct indicating the kind of error that has occurred and any precise details of that error.
pub struct AstError {
    pub kind: Kind,
    pub details: Cow<'static, str>,
}

impl AstError {
    pub fn new<I>(kind: Kind, details: I) -> AstError
    where
        I: Into<Cow<'static, str>>,
    {
        AstError {
            kind,
            details: details.into(),
        }
    }

    pub fn into_box(self) -> Box<dyn StdError> {
        match self.kind {
            Kind::Custom(err) => err,
            _ => Box::new(self),
        }
    }

    pub fn new_no_match_close_error(token: LexToken) -> AstError {
        AstError {
            kind: Kind::NoMatchClose(token),
            details: "".into(),
        }
    }
}


impl From<num::ParseIntError> for AstError {
    fn from(err: num::ParseIntError) -> AstError {
        AstError::new(Kind::ParseIntError(err), "")
    }
}

impl From<num::ParseFloatError> for AstError {
    fn from(err: num::ParseFloatError) -> AstError {
        AstError::new(Kind::ParseFloatError(err), "")
    }
}


impl fmt::Debug for AstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.details.len() > 0 {
            write!(f, "AST Error <{:?}>: {}", self.kind, self.details)
        } else {
            write!(f, "AST Error <{:?}>", self.kind)
        }
    }
}

impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.details.len() > 0 {
            write!(f, "{}: {}", self.description(), self.details)
        } else {
            write!(f, "{}", self.description())
        }
    }
}

impl StdError for AstError {
    fn description(&self) -> &str {
        match self.kind {
            Kind::Internal => "Internal Application Error",
            Kind::NoMatchClose(_) => "Not Match close Error", 
            Kind::ParseIntError(_) => "parse Int Error",
            Kind::ParseFloatError(_) => "parse Float Error",
            Kind::Custom(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match self.kind {
            _ => None,
        }
    }
}

impl<B> From<Box<B>> for AstError
where
    B: StdError + Send + Sync + 'static,
{
    fn from(err: Box<B>) -> AstError {
        AstError::new(Kind::Custom(err), "")
    }
}
