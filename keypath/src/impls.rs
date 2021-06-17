//! trait impls for std types

use std::any::Any;
use std::collections::HashMap;

use super::{FieldError, FieldErrorKind, KeyPath, PathComponent, RawKeyable, TypedKeyable};

pub struct Leaf<T> {
    _type: std::marker::PhantomData<T>,
}

impl<T> Leaf<T> {
    pub fn to_key_path_with_root<Root>(self, fields: &'static [PathComponent]) -> KeyPath<Root, T> {
        KeyPath::__conjure_from_abyss(fields)
    }
}

macro_rules! keyable_leaf {
    ($name:ty) => {
        impl RawKeyable for $name {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn get_field(&self, ident: &[PathComponent]) -> Result<&dyn RawKeyable, FieldError> {
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
                ident: &[PathComponent],
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

        impl TypedKeyable for $name {
            type PathFragment = Leaf<$name>;
            fn get() -> Leaf<$name> {
                Leaf {
                    _type: std::marker::PhantomData,
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

    fn get_field(&self, ident: &[PathComponent]) -> Result<&dyn RawKeyable, FieldError> {
        match ident.split_first() {
            None => Ok(self),
            Some((PathComponent::IndexInt(idx), rest)) => self
                .get(*idx)
                .ok_or_else(|| FieldError {
                    kind: FieldErrorKind::IndexOutOfRange(*idx),
                    type_name: std::any::type_name::<Self>(),
                    depth: rest.len(),
                })
                .and_then(|t| t.get_field(rest)),
            Some((field, rest)) => Err(FieldError {
                kind: FieldErrorKind::InvalidField(*field),
                type_name: std::any::type_name::<Self>(),
                depth: rest.len(),
            }),
        }
    }

    fn get_field_mut(
        &mut self,
        ident: &[PathComponent],
    ) -> Result<&mut dyn RawKeyable, FieldError> {
        match ident.split_first() {
            None => Ok(self),
            Some((PathComponent::IndexInt(idx), rest)) => self
                .get_mut(*idx)
                .ok_or_else(|| FieldError {
                    kind: FieldErrorKind::IndexOutOfRange(*idx),
                    type_name: std::any::type_name::<Self>(),
                    depth: rest.len(),
                })
                .and_then(|t| t.get_field_mut(rest)),
            Some((field, rest)) => Err(FieldError {
                kind: FieldErrorKind::InvalidField(*field),
                type_name: std::any::type_name::<Self>(),
                depth: rest.len(),
            }),
        }
    }
}

impl<T: TypedKeyable> TypedKeyable for Vec<T> {
    type PathFragment = VecMirror<T>;

    fn get() -> Self::PathFragment {
        VecMirror(std::marker::PhantomData)
    }
}

pub struct VecMirror<T>(std::marker::PhantomData<T>);

impl<T: TypedKeyable> VecMirror<T> {
    pub fn sequence_get(self) -> <T as TypedKeyable>::PathFragment {
        <T as TypedKeyable>::get()
    }
}

impl<K: 'static, T> RawKeyable for HashMap<K, T>
where
    T: TypedKeyable,
    K: std::cmp::Eq + std::hash::Hash + std::borrow::Borrow<str>,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_field(&self, ident: &[PathComponent]) -> Result<&dyn RawKeyable, FieldError> {
        match ident.split_first() {
            None => Ok(self),
            Some((PathComponent::IndexStr(idx), rest)) => self
                .get(*idx)
                .ok_or_else(|| FieldError {
                    kind: FieldErrorKind::MissinngKey(idx.to_string()),
                    type_name: std::any::type_name::<Self>(),
                    depth: rest.len(),
                })
                .and_then(|t| t.get_field(rest)),
            Some((field, rest)) => Err(FieldError {
                kind: FieldErrorKind::InvalidField(*field),
                type_name: std::any::type_name::<Self>(),
                depth: rest.len(),
            }),
        }
    }

    fn get_field_mut(
        &mut self,
        ident: &[PathComponent],
    ) -> Result<&mut dyn RawKeyable, FieldError> {
        match ident.split_first() {
            None => Ok(self),
            Some((PathComponent::IndexStr(idx), rest)) => self
                .get_mut(idx)
                .ok_or_else(|| FieldError {
                    kind: FieldErrorKind::MissinngKey(idx.to_string()),
                    type_name: std::any::type_name::<Self>(),
                    depth: rest.len(),
                })
                .and_then(|t| t.get_field_mut(rest)),
            Some((field, rest)) => Err(FieldError {
                kind: FieldErrorKind::InvalidField(*field),
                type_name: std::any::type_name::<Self>(),
                depth: rest.len(),
            }),
        }
    }
}

impl<K, T> TypedKeyable for HashMap<K, T>
where
    K: std::cmp::Eq + std::hash::Hash + std::borrow::Borrow<str> + 'static,
    T: TypedKeyable,
{
    type PathFragment = HashMapMirror<K, T>;

    fn get() -> Self::PathFragment {
        HashMapMirror(std::marker::PhantomData, std::marker::PhantomData)
    }
}

pub struct HashMapMirror<K, T>(std::marker::PhantomData<K>, std::marker::PhantomData<T>);

impl<K, T: TypedKeyable> HashMapMirror<K, T> {
    pub fn map_get(self) -> <T as TypedKeyable>::PathFragment {
        <T as TypedKeyable>::get()
    }
}
