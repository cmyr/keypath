//! trait impls for std types

use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;

use super::internals::{PathComponent, RawKeyable};
use super::{FieldError, FieldErrorKind, KeyPath, Keyable};

pub struct Leaf<T> {
    _type: PhantomData<T>,
}

impl<T> Leaf<T> {
    pub const fn new() -> Self {
        Leaf { _type: PhantomData }
    }

    pub const fn to_key_path_with_root<Root>(
        &self,
        fields: &'static [PathComponent],
    ) -> KeyPath<Root, T> {
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

        impl Keyable for $name {
            type Mirror = Leaf<$name>;
            fn mirror() -> Leaf<$name> {
                Leaf { _type: PhantomData }
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

macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<$($T: RawKeyable),+> RawKeyable for ($($T,)+) {
                fn as_any(&self) -> &dyn Any {
                    self
                }
                fn as_any_mut(&mut self) -> &mut dyn Any {
                    self
                }
                fn get_field(&self, ident: &[PathComponent]) -> Result<&dyn RawKeyable, FieldError> {
                    match ident.split_first() {
                        None => Ok(self),
                        $( Some((PathComponent::Unnamed($idx), rest)) => self.$idx.get_field(rest),)+
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
                        $( Some((PathComponent::Unnamed($idx), rest)) => self.$idx.get_field_mut(rest),)+
                        Some((head, rest)) => {
                            Err(FieldErrorKind::InvalidField(head.to_owned())
                                .into_error(self, rest.len()))
                        }
                    }
                }
            }

            pub struct $Tuple<$($T),+>($(PhantomData<$T>),+);


            impl<$($T),+> $Tuple<$($T,)+> {
                pub const fn new() -> Self {
                    $Tuple($( make_phantom::<$T>() ),+)
                }
            }

            impl<$($T: RawKeyable),+> Keyable for ($($T,)+) {
                type Mirror = $Tuple<$($T),+>;
                fn mirror() -> Self::Mirror {
                    $Tuple::<$($T),+>::new()
                }

            }


        )+
    }
}

const fn make_phantom<T>() -> PhantomData<T> {
    PhantomData
}
tuple_impls! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
}

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

impl<T: Keyable> Keyable for Vec<T> {
    type Mirror = VecMirror<T>;

    fn mirror() -> Self::Mirror {
        VecMirror(PhantomData)
    }
}

pub struct VecMirror<T>(PhantomData<T>);

impl<T> VecMirror<T> {
    pub const fn new() -> Self {
        VecMirror(PhantomData)
    }
}

impl<T: Keyable> VecMirror<T> {
    pub fn sequence_get(self) -> <T as Keyable>::Mirror {
        <T as Keyable>::mirror()
    }
}

impl<K: 'static, T> RawKeyable for HashMap<K, T>
where
    T: Keyable,
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

impl<K, T> Keyable for HashMap<K, T>
where
    K: std::cmp::Eq + std::hash::Hash + std::borrow::Borrow<str> + 'static,
    T: Keyable,
{
    type Mirror = HashMapMirror<K, T>;

    fn mirror() -> Self::Mirror {
        HashMapMirror(PhantomData, PhantomData)
    }
}

pub struct HashMapMirror<K, T>(PhantomData<K>, PhantomData<T>);

impl<K, T> HashMapMirror<K, T> {
    pub const fn new() -> Self {
        HashMapMirror(PhantomData, PhantomData)
    }
}

impl<K, T: Keyable> HashMapMirror<K, T> {
    pub fn map_get(self) -> <T as Keyable>::Mirror {
        <T as Keyable>::mirror()
    }
}
