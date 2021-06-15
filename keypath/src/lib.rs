mod impls;

pub use keypath_derive::Keyable;
use std::any::Any;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct SimplePath<T: 'static> {
    fields: &'static [Field],
    _type: PhantomData<T>,
}

impl<T: 'static> SimplePath<T> {
    pub fn new(fields: &'static [Field]) -> SimplePath<T> {
        SimplePath {
            fields,
            _type: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Field {
    Ord(usize),
    Name(&'static str),
}

pub trait RawKeyable: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_field(&self, ident: &[Field]) -> Result<&dyn RawKeyable, FieldError>;
    fn get_field_mut(&mut self, ident: &[Field]) -> Result<&mut dyn RawKeyable, FieldError>;
}

pub trait Keyable: RawKeyable {
    fn item_at_path<T>(&self, path: &SimplePath<T>) -> Result<&T, FieldError> {
        self.get_field(path.fields)
            //.ok()
            //FIXME: no unwrap here, some new more expresesive error type instead
            .map(|t| t.as_any().downcast_ref().unwrap())
    }

    fn set_item_at_path<T>(&mut self, path: &SimplePath<T>, new: T) -> Result<(), FieldError> {
        *self
            .get_field_mut(path.fields)?
            .as_any_mut()
            .downcast_mut()
            .unwrap() = new;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum FieldErrorKind {
    IndexOutOfRange(usize),
    InvalidField(Field),
}

#[derive(Debug, Clone)]
pub struct FieldError {
    kind: FieldErrorKind,
    type_name: &'static str,
    // the number of *remaining* fields at which the error occured
    depth: usize,
}

impl FieldErrorKind {
    pub fn into_error<T>(self, _source: &T, depth: usize) -> FieldError {
        FieldError {
            kind: self,
            type_name: std::any::type_name::<T>(),
            depth,
        }
    }
}
