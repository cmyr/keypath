mod error;
mod impls;
pub mod internals;

pub use error::{FieldError, FieldErrorKind};
pub use keypath_derive::{keypath, Keyable};

use std::any::Any;
use std::marker::PhantomData;

/// A non-fallible keypath.
pub struct KeyPath<Root: ?Sized, Value: 'static> {
    fields: &'static [internals::PathComponent],
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
    pub fn __conjure_from_abyss(fields: &'static [internals::PathComponent]) -> Self {
        KeyPath {
            fields,
            _root: PhantomData,
            _value: PhantomData,
        }
    }
}

pub trait Keyable: internals::RawKeyable {
    /// A type that describes properties on the inner type, for compile-time checking.
    ///
    /// This is the worst part of the code right now? We generate structs with magic
    /// names for each Keyable type.
    type Mirror;

    /// Return an instance of this type's mirror.
    fn mirror() -> Self::Mirror;

    //TODO: this is a bit of a mess, and I don't know what methods we will want
    //or need. Having partial keypaths or keypaths that are failable seems reasonable,
    //but I don't know what the types are going to look like yet.
    fn try_any_at_path(
        &self,
        path: impl AsRef<[internals::PathComponent]>,
    ) -> Result<&dyn Any, FieldError> {
        self.get_field(path.as_ref())
            .map(internals::RawKeyable::as_any)
    }

    fn try_any_at_path_mut(
        &mut self,
        path: impl AsRef<[internals::PathComponent]>,
    ) -> Result<&mut dyn Any, FieldError> {
        self.get_field_mut(path.as_ref())
            .map(internals::RawKeyable::as_any_mut)
    }

    //NOTE: these two methods are intended in cases where the keypath has not been
    //validated, but currently we don't really support creating invalid keypaths.
    fn try_item_at_path<T>(&self, path: &KeyPath<Self, T>) -> Result<&T, FieldError> {
        self.try_any_at_path(path)
            ////FIXME: no unwrap here, some new more expresesive error type instead
            .map(|t| t.downcast_ref().unwrap())
    }

    fn try_item_at_path_mut<T>(&mut self, path: &KeyPath<Self, T>) -> Result<&mut T, FieldError> {
        self.try_any_at_path_mut(path)
            //FIXME: no unwrap here, some new more expresesive error type instead
            .map(|t| t.downcast_mut().unwrap())
    }

    fn item_at_path<T>(&self, path: &KeyPath<Self, T>) -> &T {
        self.try_item_at_path(path).unwrap()
    }

    fn item_at_path_mut<T>(&mut self, path: &KeyPath<Self, T>) -> &mut T {
        self.try_item_at_path_mut(path).unwrap()
    }
}

impl<Root: ?Sized, Value: 'static> AsRef<[internals::PathComponent]> for KeyPath<Root, Value> {
    fn as_ref(&self) -> &[internals::PathComponent] {
        &self.fields
    }
}
