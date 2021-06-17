use super::internals::PathComponent;

#[derive(Debug, Clone)]
pub enum FieldErrorKind {
    IndexOutOfRange(usize),
    MissinngKey(String),
    InvalidField(PathComponent),
}

#[derive(Debug, Clone)]
pub struct FieldError {
    pub(crate) kind: FieldErrorKind,
    pub(crate) type_name: &'static str,
    // the number of *remaining* fields at which the error occured
    pub(crate) depth: usize,
}

impl std::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
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
