mod error;
mod impls;
pub mod internals;

pub use error::{FieldError, FieldErrorKind};
pub use keypath_derive::{keypath, Keyable};

use std::any::Any;
use std::marker::PhantomData;

use internals::PathComponent;

//TODO: change name? delete?
/// A failable typed keypath.
#[derive(Debug, Clone, Copy)]
pub struct SimplePath<T: 'static> {
    fields: &'static [PathComponent],
    _type: PhantomData<T>,
}

impl<T: 'static> SimplePath<T> {
    pub fn new(fields: &'static [PathComponent]) -> SimplePath<T> {
        SimplePath {
            fields,
            _type: PhantomData,
        }
    }
}

/// A non-fallible keypath.
pub struct KeyPath<Root, Value> {
    fields: &'static [PathComponent],
    _root: PhantomData<Root>,
    _value: PhantomData<Value>,
}

impl<Root, Value> KeyPath<Root, Value> {
    /// Create a new typed `KeyPath` from the provided fields.
    ///
    /// This method does not ensure the path is valid; it is intended
    /// to be called after a path has been type-checked, presumably in the
    /// context of a proc_macro.
    #[doc(hidden)]
    pub fn __conjure_from_abyss(fields: &'static [PathComponent]) -> Self {
        KeyPath {
            fields,
            _root: PhantomData,
            _value: PhantomData,
        }
    }
}

/// A trait for types that expose their properties via keypath.
pub trait RawKeyable: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_field(&self, ident: &[PathComponent]) -> Result<&dyn RawKeyable, FieldError>;
    fn get_field_mut(&mut self, ident: &[PathComponent])
        -> Result<&mut dyn RawKeyable, FieldError>;
}

//TODO: obsolete? not obsolete? replace with TypedKeyable? combine them?
pub trait Keyable: RawKeyable {
    fn item_at_path1<T>(&self, path: &SimplePath<T>) -> Result<&T, FieldError> {
        self.get_field(path.fields)
            //.ok()
            //FIXME: no unwrap here, some new more expresesive error type instead
            .map(|t| t.as_any().downcast_ref().unwrap())
    }

    fn set_item_at_path1<T>(&mut self, path: &SimplePath<T>, new: T) -> Result<(), FieldError> {
        *self
            .get_field_mut(path.fields)?
            .as_any_mut()
            .downcast_mut()
            .unwrap() = new;
        Ok(())
    }
}

pub trait TypedKeyable: RawKeyable + Sized {
    /// A type that describes properties on the inner type, for compile-time checking.
    ///
    /// This is the worst part of the code right now? We generate structs with magic
    /// names for each Keyable type.
    type PathFragment;

    fn get() -> Self::PathFragment;

    fn item_at_path<T: 'static>(&self, path: &KeyPath<Self, T>) -> &T {
        self.get_field(path.fields)
            //.ok()
            //FIXME: no unwrap here, some new more expresesive error type instead
            .map(|t| t.as_any().downcast_ref().unwrap())
            .unwrap()
    }

    fn item_at_path_mut<T: 'static>(&mut self, path: &KeyPath<Self, T>) -> &mut T {
        self.get_field_mut(path.fields)
            //.ok()
            //FIXME: no unwrap here, some new more expresesive error type instead
            .map(|t| t.as_any_mut().downcast_mut().unwrap())
            .unwrap()
    }
}
