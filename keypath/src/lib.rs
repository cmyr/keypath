//! Derivable references to arbitarily nested fields.
//!
//! This crate contains a basic implementation of 'keypaths', a mechanism
//! for creating paths of references to the properties of objects that can
//! be used to get and set their underlying values.
//!
//! The general idea and design is strongly influenced by keypaths in Swift.
//!
//! To work with keypaths, types must implement the [`Keyable`][] trait; in most
//! cases this will be derived.
//!
//! [`KeyPath`][] instances can then be created with the [`keypath!`][] macro.
//!
//! # Examples
//!
//! ```
//! use keypath::{Keyable, KeyPath, keypath};
//!
//! #[derive(Keyable)]
//! struct Person {
//!     name: String,
//!     friends: Vec<String>,
//!     size: Size,
//! }
//!
//! #[derive(Keyable)]
//! struct Size {
//!     big: bool,
//!     heft: u8,
//! }
//!
//! let mut person = Person {
//!     name: "coco".into(),
//!     friends: vec!["eli".into(), "nico".into(), "yaya".into()],
//!     size: Size { big: false, heft: 45 }
//! };
//!
//! let first_friend: KeyPath<Person, String> = keypath!(Person.friends[0]);
//! let heft = keypath!(Person.size.heft);
//!
//! assert_eq!(person[&first_friend], "eli");
//!
//! // mutation:
//! person[&heft] = 101;
//! assert_eq!(person.size.heft, 101);
//!
//! ```

mod error;
mod impls;
pub mod internals;

pub use error::{FieldError, FieldErrorKind};
pub use keypath_proc_macros::{keypath, Keyable};

use std::any::Any;
use std::borrow::Cow;
use std::marker::PhantomData;

/// A non-fallible keypath.
pub struct KeyPath<Root: ?Sized, Value: 'static> {
    partial: PartialKeyPath<Root>,
    _value: PhantomData<Value>,
}

// we don't really use this yet
#[doc(hidden)]
/// A keypath for a known route, but which doesn't know the destination type.
#[derive(Debug)]
pub struct PartialKeyPath<Root: ?Sized> {
    fields: Cow<'static, [internals::PathComponent]>,
    _root: PhantomData<Root>,
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
            partial: PartialKeyPath {
                fields: Cow::Borrowed(fields),
                _root: PhantomData,
            },
            _value: PhantomData,
        }
    }

    /// Create a new `KeyPath` by combining two routes.
    ///
    /// The final type of the first route must be the first type of the second
    /// route; assuming both paths were created with the [`keypath!`] macro,
    /// the resulting path must be valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use keypath::{Keyable, KeyPath, keypath};
    ///
    /// #[derive(Keyable)]
    /// struct Person {
    ///     name: String,
    ///     friends: Vec<String>,
    ///     size: Size,
    /// }
    ///
    /// #[derive(Keyable)]
    /// struct Size {
    ///     big: bool,
    ///     heft: u8,
    /// }
    ///
    /// let mut person = Person {
    ///     name: "coco".into(),
    ///     friends: vec!["eli".into(), "nico".into(), "yaya".into()],
    ///     size: Size { big: false, heft: 45 }
    /// };
    ///
    /// let size = keypath!(Person.size);
    /// let heft = keypath!(Size.heft);
    /// let combined = size.append(&heft);
    ///
    /// assert_eq!(person[&combined], 45);
    /// ```
    pub fn append<T>(&self, other: &KeyPath<Value, T>) -> KeyPath<Root, T> {
        let mut partial = self.partial.clone();
        partial
            .fields
            .to_mut()
            .extend(other.partial.fields.iter().clone());
        KeyPath {
            partial,
            _value: other._value,
        }
    }
}

/// A trait for types that can be indexed with keypaths.
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
    /// Attempt to traverse a series of `PathComponent`s, returning an `&dyn Any`
    /// if successful.
    fn try_any_at_path(
        &self,
        path: impl AsRef<[internals::PathComponent]>,
    ) -> Result<&dyn Any, FieldError> {
        self.get_field(path.as_ref())
            .map(internals::RawKeyable::as_any)
    }

    /// Attempt to traverse a series of `PathComponent`s, returning an `&mut dyn Any`
    /// if successful.
    fn try_any_at_path_mut(
        &mut self,
        path: impl AsRef<[internals::PathComponent]>,
    ) -> Result<&mut dyn Any, FieldError> {
        self.get_field_mut(path.as_ref())
            .map(internals::RawKeyable::as_any_mut)
    }

    //NOTE: these two methods are intended in cases where the keypath has not been
    //validated, but currently we don't really support creating invalid keypaths.
    #[doc(hidden)]
    fn try_item_at_path<T>(&self, path: &KeyPath<Self, T>) -> Result<&T, FieldError> {
        self.try_any_at_path(path)
            ////FIXME: no unwrap here, some new more expresesive error type instead
            .map(|t| t.downcast_ref().unwrap())
    }

    #[doc(hidden)]
    fn try_item_at_path_mut<T>(&mut self, path: &KeyPath<Self, T>) -> Result<&mut T, FieldError> {
        self.try_any_at_path_mut(path)
            //FIXME: no unwrap here, some new more expresesive error type instead
            .map(|t| t.downcast_mut().unwrap())
    }

    /// Get a reference to the value at the provided path.
    ///
    /// You generally won't need to call this, since you can use `[index]`
    /// syntax instead.
    ///
    /// Assuming the path was constructed with the [`keypath!`] macro, this
    /// method will not fail.
    ///
    ///
    /// # Panics
    ///
    /// I lied, it will fail if you provide an index into a collection and
    /// that item does not exist.
    fn item_at_path<T>(&self, path: &KeyPath<Self, T>) -> &T {
        self.try_item_at_path(path).unwrap()
    }

    /// Get a mutable reference to the value at the provided path.
    ///
    /// You generally won't need to call this, since you can use `[index]`
    /// syntax instead.
    ///
    /// Assuming the path was constructed with the [`keypath!`] macro, this
    /// method will not fail.
    ///
    ///
    /// # Panics
    ///
    /// I lied, it will fail if you provide an index into a collection and
    /// that item does not exist.
    fn item_at_path_mut<T>(&mut self, path: &KeyPath<Self, T>) -> &mut T {
        self.try_item_at_path_mut(path).unwrap()
    }
}

impl<Root: ?Sized, Value: 'static> AsRef<[internals::PathComponent]> for KeyPath<Root, Value> {
    fn as_ref(&self) -> &[internals::PathComponent] {
        self.partial.fields.as_ref()
    }
}

impl<R: ?Sized> Clone for PartialKeyPath<R> {
    fn clone(&self) -> Self {
        PartialKeyPath {
            fields: self.fields.clone(),
            _root: PhantomData,
        }
    }
}

