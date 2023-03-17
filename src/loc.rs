use std::sync::Arc;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SourceRange {
    pub begin: BareSourceLocation,
    pub end: BareSourceLocation,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct BareSourceLocation {
    pub offset: usize,
    pub file: Arc<str>,
    pub line: usize,
    pub col: usize,
    pub tok_len: usize,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Production {
    pub file: Arc<str>,
    pub slice: Arc<str>,
    pub stack: u32,
}

