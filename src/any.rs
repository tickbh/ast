use std::ops::{Add, Sub};




#[derive(Clone, Debug)]
pub enum AstAny {
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    U128(u128),
    I128(i128),
    Isize(isize),
    Usize(usize),
    F32(f32),
    F64(f64),
    Str(String),
    Char(char),
    Other(Vec<u8>),
    Unsport,
    Unknow,
}

impl Add for &AstAny {
    type Output = AstAny;

    fn add(self, other: Self) -> AstAny {
        match (self, other) {
            (AstAny::Bool(a), AstAny::Bool(b)) => {
                AstAny::Bool(*a && *b)
            },
            (AstAny::U8(a), AstAny::U8(b)) => {
                AstAny::U8(a.overflowing_add(*b).0)
            },
            (AstAny::I8(a), AstAny::I8(b)) => {
                AstAny::I8(a.overflowing_add(*b).0)
            },
            (AstAny::U16(a), AstAny::U16(b)) => {
                AstAny::U16(a.overflowing_add(*b).0)
            },
            (AstAny::I16(a), AstAny::I16(b)) => {
                AstAny::I16(a.overflowing_add(*b).0)
            },
            (AstAny::U32(a), AstAny::U32(b)) => {
                AstAny::U32(a.overflowing_add(*b).0)
            },
            (AstAny::I32(a), AstAny::I32(b)) => {
                AstAny::I32(a.overflowing_add(*b).0)
            },
            (AstAny::U64(a), AstAny::U64(b)) => {
                AstAny::U64(a.overflowing_add(*b).0)
            },
            (AstAny::I64(a), AstAny::I64(b)) => {
                AstAny::I64(a.overflowing_add(*b).0)
            },
            (AstAny::U128(a), AstAny::U128(b)) => {
                AstAny::U128(a.overflowing_add(*b).0)
            },
            (AstAny::I128(a), AstAny::I128(b)) => {
                AstAny::I128(a.overflowing_add(*b).0)
            },
            (AstAny::Usize(a), AstAny::Usize(b)) => {
                AstAny::Usize(a.overflowing_add(*b).0)
            },
            (AstAny::Isize(a), AstAny::Isize(b)) => {
                AstAny::Isize(a.overflowing_add(*b).0)
            },
            (AstAny::F32(a), AstAny::F32(b)) => {
                AstAny::F32(a + b)
            },
            (AstAny::F64(a), AstAny::F64(b)) => {
                AstAny::F64(a + b)
            },
            (AstAny::Str(a), AstAny::Str(b)) => {
                AstAny::Str(a.to_owned() + b)
            },
            _ => {
                AstAny::Unsport
            }
        }
    }
}

impl Sub for &AstAny {
    type Output = AstAny;

    fn sub(self, other: Self) -> AstAny {
        match (self, other) {
            (AstAny::Bool(a), AstAny::Bool(b)) => {
                AstAny::Bool(*a && *b)
            },
            (AstAny::U8(a), AstAny::U8(b)) => {
                AstAny::U8(a.overflowing_add(*b).0)
            },
            (AstAny::I8(a), AstAny::I8(b)) => {
                AstAny::I8(a.overflowing_add(*b).0)
            },
            (AstAny::U16(a), AstAny::U16(b)) => {
                AstAny::U16(a.overflowing_add(*b).0)
            },
            (AstAny::I16(a), AstAny::I16(b)) => {
                AstAny::I16(a.overflowing_add(*b).0)
            },
            (AstAny::U32(a), AstAny::U32(b)) => {
                AstAny::U32(a.overflowing_add(*b).0)
            },
            (AstAny::I32(a), AstAny::I32(b)) => {
                AstAny::I32(a.overflowing_add(*b).0)
            },
            (AstAny::U64(a), AstAny::U64(b)) => {
                AstAny::U64(a.overflowing_add(*b).0)
            },
            (AstAny::I64(a), AstAny::I64(b)) => {
                AstAny::I64(a.overflowing_add(*b).0)
            },
            (AstAny::U128(a), AstAny::U128(b)) => {
                AstAny::U128(a.overflowing_add(*b).0)
            },
            (AstAny::I128(a), AstAny::I128(b)) => {
                AstAny::I128(a.overflowing_add(*b).0)
            },
            (AstAny::Usize(a), AstAny::Usize(b)) => {
                AstAny::Usize(a.overflowing_add(*b).0)
            },
            (AstAny::Isize(a), AstAny::Isize(b)) => {
                AstAny::Isize(a.overflowing_add(*b).0)
            },
            (AstAny::F32(a), AstAny::F32(b)) => {
                AstAny::F32(a + b)
            },
            (AstAny::F64(a), AstAny::F64(b)) => {
                AstAny::F64(a + b)
            },
            (AstAny::Str(a), AstAny::Str(b)) => {
                AstAny::Str(a.to_owned() + b)
            },
            _ => {
                AstAny::Unsport
            }
        }
    }
}

