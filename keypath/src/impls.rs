//! trait impls for std types

use super::{Field, FieldError, FieldErrorKind, RawKeyable};
use std::any::Any;

macro_rules! keyable_leaf {
    ($name:ty) => {
        impl RawKeyable for $name {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn get_field(&self, ident: &[Field]) -> Result<&dyn RawKeyable, FieldError> {
                match ident.split_first() {
                    None => Ok(self),
                    Some((head, rest)) => {
                        Err(FieldErrorKind::InvalidField(head.to_owned())
                            .into_error(self, rest.len()))
                    }
                }
            }

            fn get_field_mut(
                &mut self,
                ident: &[Field],
            ) -> Result<&mut dyn RawKeyable, FieldError> {
                match ident.split_first() {
                    None => Ok(self),
                    Some((head, rest)) => {
                        Err(FieldErrorKind::InvalidField(head.to_owned())
                            .into_error(self, rest.len()))
                    }
                }
            }
        }
    };
}

keyable_leaf!(bool);
keyable_leaf!(char);

keyable_leaf!(u8);
keyable_leaf!(u16);
keyable_leaf!(u32);
keyable_leaf!(u64);
keyable_leaf!(u128);
keyable_leaf!(usize);

keyable_leaf!(i8);
keyable_leaf!(i16);
keyable_leaf!(i32);
keyable_leaf!(i64);
keyable_leaf!(i128);
keyable_leaf!(isize);

keyable_leaf!(f32);
keyable_leaf!(f64);

keyable_leaf!(String);

impl<T: RawKeyable> RawKeyable for Vec<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_field(&self, ident: &[Field]) -> Result<&dyn RawKeyable, FieldError> {
        match ident.split_first() {
            None => Ok(self),
            Some((Field::Ord(idx), rest)) => self
                .get(*idx)
                .ok_or_else(|| FieldError {
                    kind: FieldErrorKind::IndexOutOfRange(*idx),
                    type_name: std::any::type_name::<Self>(),
                    depth: rest.len(),
                })
                .and_then(|t| t.get_field(rest)),
            Some((field, rest)) => Err(FieldError {
                kind: FieldErrorKind::InvalidField(field.clone()),
                type_name: std::any::type_name::<Self>(),
                depth: rest.len(),
            }),
        }
    }

    fn get_field_mut(&mut self, ident: &[Field]) -> Result<&mut dyn RawKeyable, FieldError> {
        match ident.split_first() {
            None => Ok(self),
            Some((Field::Ord(idx), rest)) => self
                .get_mut(*idx)
                .ok_or_else(|| FieldError {
                    kind: FieldErrorKind::IndexOutOfRange(*idx),
                    type_name: std::any::type_name::<Self>(),
                    depth: rest.len(),
                })
                .and_then(|t| t.get_field_mut(rest)),
            Some((field, rest)) => Err(FieldError {
                kind: FieldErrorKind::InvalidField(field.clone()),
                type_name: std::any::type_name::<Self>(),
                depth: rest.len(),
            }),
        }
    }
}
