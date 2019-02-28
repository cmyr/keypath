
pub enum Value<'a> {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Isize(isize),
    F32(f32),
    F64(f64),
    Char(char),
    Str(&'a str),
    //Seq(&'a [Value<'a>]),
    //Bytes(&'a [u8]),
}

pub type Error = &'static str;

impl<'a> Value<'a>  {
    pub fn try_as_type<T: TryFromValue<'a> + 'a>(self) -> Result<T, Error> {
        T::try_from_value(self)
    }
}

pub trait AsValue {
    fn as_value<'a>(&'a self) -> Value<'a>;
}

pub trait TryFromValue<'a>: Sized + 'a {
    fn try_from_value(value: Value<'a>) -> Result<Self, Error>;
}

impl AsValue for String {
    fn as_value<'a>(&'a self) -> Value<'a> {
        Value::Str(self.as_str())
    }
}

impl<'a> TryFromValue<'a> for &'a str {
    fn try_from_value(value: Value<'a>) -> Result<Self, Error> {
        match value {
            Value::Str(s) => Ok(s),
            _other => Err("incorrect value, expected &str"),
        }
    }
}

macro_rules! impl_as_value_copy {
    ($t:ty, $variant:ident) => {
        impl AsValue for $t {
            fn as_value<'a>(&'a self) -> Value<'a> {
                Value::$variant(*self)
            }
        }
    };
}

macro_rules! impl_try_from_value {
    ($t:ty, $variant:ident) => {
        impl<'a> TryFromValue<'a> for $t {
            fn try_from_value(value: Value<'a>) -> Result<Self, Error> {
                match value {
                    Value::$variant(val) => Ok(val),
                    _other => Err(stringify!(incorrect value, expected $t)),
                }
            }
        }
    };
}


impl_as_value_copy!(bool, Bool);
impl_as_value_copy!(u8, U8);
impl_as_value_copy!(u16, U16);
impl_as_value_copy!(u32, U32);
impl_as_value_copy!(u64, U64);
impl_as_value_copy!(usize, Usize);
impl_as_value_copy!(i8, I8);
impl_as_value_copy!(i16, I16);
impl_as_value_copy!(i32, I32);
impl_as_value_copy!(i64, I64);
impl_as_value_copy!(isize, Isize);
impl_as_value_copy!(f32, F32);
impl_as_value_copy!(f64, F64);
impl_as_value_copy!(char, Char);

impl_try_from_value!(bool, Bool);
impl_try_from_value!(u8, U8);
impl_try_from_value!(u16, U16);
impl_try_from_value!(u32, U32);
impl_try_from_value!(u64, U64);
impl_try_from_value!(usize, Usize);
impl_try_from_value!(i8, I8);
impl_try_from_value!(i16, I16);
impl_try_from_value!(i32, I32);
impl_try_from_value!(i64, I64);
impl_try_from_value!(isize, Isize);
impl_try_from_value!(f32, F32);
impl_try_from_value!(f64, F64);
impl_try_from_value!(char, Char);
