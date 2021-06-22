use std::any::Any;

use super::FieldError;

/// A trait for types that expose their properties via keypath.
///
/// All of the dynamism and traversal logic happens here; its split into a
/// separate trait for object safety.
pub trait RawKeyable: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_field(&self, ident: &[PathComponent]) -> Result<&dyn RawKeyable, FieldError>;
    fn get_field_mut(&mut self, ident: &[PathComponent])
        -> Result<&mut dyn RawKeyable, FieldError>;
}

/// A component of a keypath.
#[derive(Debug, Clone, Copy)]
pub enum PathComponent {
    /// An unnamed field, such as on a tuple or tuple struct
    Unnamed(usize),
    /// A named field.
    Named(&'static str),
    /// An index into a sequence, such as a vec.
    IndexInt(usize),
    /// An index into a map with string keys.
    IndexStr(&'static str),
}
