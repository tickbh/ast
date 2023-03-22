use std::ops::{Add, Sub};




#[derive(Clone, Debug)]
pub enum LangAny {
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

impl Add for &LangAny {
    type Output = LangAny;

    fn add(self, other: Self) -> LangAny {
        match (self, other) {
            (LangAny::Bool(a), LangAny::Bool(b)) => {
                LangAny::Bool(*a && *b)
            },
            (LangAny::U8(a), LangAny::U8(b)) => {
                LangAny::U8(a.overflowing_add(*b).0)
            },
            (LangAny::I8(a), LangAny::I8(b)) => {
                LangAny::I8(a.overflowing_add(*b).0)
            },
            (LangAny::U16(a), LangAny::U16(b)) => {
                LangAny::U16(a.overflowing_add(*b).0)
            },
            (LangAny::I16(a), LangAny::I16(b)) => {
                LangAny::I16(a.overflowing_add(*b).0)
            },
            (LangAny::U32(a), LangAny::U32(b)) => {
                LangAny::U32(a.overflowing_add(*b).0)
            },
            (LangAny::I32(a), LangAny::I32(b)) => {
                LangAny::I32(a.overflowing_add(*b).0)
            },
            (LangAny::U64(a), LangAny::U64(b)) => {
                LangAny::U64(a.overflowing_add(*b).0)
            },
            (LangAny::I64(a), LangAny::I64(b)) => {
                LangAny::I64(a.overflowing_add(*b).0)
            },
            (LangAny::U128(a), LangAny::U128(b)) => {
                LangAny::U128(a.overflowing_add(*b).0)
            },
            (LangAny::I128(a), LangAny::I128(b)) => {
                LangAny::I128(a.overflowing_add(*b).0)
            },
            (LangAny::Usize(a), LangAny::Usize(b)) => {
                LangAny::Usize(a.overflowing_add(*b).0)
            },
            (LangAny::Isize(a), LangAny::Isize(b)) => {
                LangAny::Isize(a.overflowing_add(*b).0)
            },
            (LangAny::F32(a), LangAny::F32(b)) => {
                LangAny::F32(a + b)
            },
            (LangAny::F64(a), LangAny::F64(b)) => {
                LangAny::F64(a + b)
            },
            (LangAny::Str(a), LangAny::Str(b)) => {
                LangAny::Str(a.to_owned() + b)
            },
            _ => {
                LangAny::Unsport
            }
        }
    }
}

impl Sub for &LangAny {
    type Output = LangAny;

    fn sub(self, other: Self) -> LangAny {
        match (self, other) {
            (LangAny::Bool(a), LangAny::Bool(b)) => {
                LangAny::Bool(*a && *b)
            },
            (LangAny::U8(a), LangAny::U8(b)) => {
                LangAny::U8(a.overflowing_add(*b).0)
            },
            (LangAny::I8(a), LangAny::I8(b)) => {
                LangAny::I8(a.overflowing_add(*b).0)
            },
            (LangAny::U16(a), LangAny::U16(b)) => {
                LangAny::U16(a.overflowing_add(*b).0)
            },
            (LangAny::I16(a), LangAny::I16(b)) => {
                LangAny::I16(a.overflowing_add(*b).0)
            },
            (LangAny::U32(a), LangAny::U32(b)) => {
                LangAny::U32(a.overflowing_add(*b).0)
            },
            (LangAny::I32(a), LangAny::I32(b)) => {
                LangAny::I32(a.overflowing_add(*b).0)
            },
            (LangAny::U64(a), LangAny::U64(b)) => {
                LangAny::U64(a.overflowing_add(*b).0)
            },
            (LangAny::I64(a), LangAny::I64(b)) => {
                LangAny::I64(a.overflowing_add(*b).0)
            },
            (LangAny::U128(a), LangAny::U128(b)) => {
                LangAny::U128(a.overflowing_add(*b).0)
            },
            (LangAny::I128(a), LangAny::I128(b)) => {
                LangAny::I128(a.overflowing_add(*b).0)
            },
            (LangAny::Usize(a), LangAny::Usize(b)) => {
                LangAny::Usize(a.overflowing_add(*b).0)
            },
            (LangAny::Isize(a), LangAny::Isize(b)) => {
                LangAny::Isize(a.overflowing_add(*b).0)
            },
            (LangAny::F32(a), LangAny::F32(b)) => {
                LangAny::F32(a + b)
            },
            (LangAny::F64(a), LangAny::F64(b)) => {
                LangAny::F64(a + b)
            },
            (LangAny::Str(a), LangAny::Str(b)) => {
                LangAny::Str(a.to_owned() + b)
            },
            _ => {
                LangAny::Unsport
            }
        }
    }
}

