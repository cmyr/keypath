pub use keypath_derive::Keyable;
use std::any::Any;
use std::marker::PhantomData;

//mod lib2;
//mod json_keypath;
//mod lib3;
//mod newstart;
//mod newstart_index;
//mod value;

#[derive(Debug, Clone, Copy)]
pub struct SimplePath<T: 'static> {
    fields: &'static [Field],
    _type: PhantomData<T>,
}

impl<T: 'static> SimplePath<T> {
    fn new(fields: &'static [Field]) -> SimplePath<T> {
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
                    Some((head, rest)) => Err(FieldError {
                        kind: FieldErrorKind::InvalidField(head.to_owned()),
                        type_name: std::any::type_name::<Self>(),
                        depth: rest.len(),
                    }),
                }
            }

            fn get_field_mut(
                &mut self,
                ident: &[Field],
            ) -> Result<&mut dyn RawKeyable, FieldError> {
                match ident.split_first() {
                    None => Ok(self),
                    Some((head, rest)) => Err(FieldError {
                        kind: FieldErrorKind::InvalidField(head.to_owned()),
                        type_name: std::any::type_name::<Self>(),
                        depth: rest.len(),
                    }),
                }
            }
        }
    };
}

keyable_leaf!(String);
keyable_leaf!(f64);

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

struct DemoStruct {
    friends: Vec<DemoPerson>,
}

//#[derive(Keyable)]
struct DemoPerson {
    name: String,
    magnitude: f64,
}

impl RawKeyable for DemoPerson {
    fn get_field(&self, ident: &[Field]) -> Result<&dyn RawKeyable, FieldError> {
        match ident.split_first() {
            None => Ok(self),
            Some((Field::Name("name"), rest)) => self.name.get_field(rest),
            Some((Field::Name("magnitude"), rest)) => self.magnitude.get_field(rest),
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
            Some((Field::Name("name"), rest)) => self.name.get_field_mut(rest),
            Some((Field::Name("magnitude"), rest)) => self.magnitude.get_field_mut(rest),
            Some((field, rest)) => Err(FieldError {
                kind: FieldErrorKind::InvalidField(field.clone()),
                type_name: std::any::type_name::<Self>(),
                depth: rest.len(),
            }),
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &(dyn Any + 'static) {
        self
    }
}

impl Keyable for DemoPerson {}

impl RawKeyable for DemoStruct {
    fn get_field(&self, ident: &[Field]) -> Result<&dyn RawKeyable, FieldError> {
        match ident.split_first() {
            None => Ok(self),
            Some((Field::Name("friends"), rest)) => self.friends.get_field(rest),
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
            Some((Field::Name("friends"), rest)) => self.friends.get_field_mut(rest),
            Some((field, rest)) => Err(FieldError {
                kind: FieldErrorKind::InvalidField(field.clone()),
                type_name: std::any::type_name::<Self>(),
                depth: rest.len(),
            }),
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &(dyn Any + 'static) {
        self
    }
}

impl Keyable for DemoStruct {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_keypath() {
        let mut person = DemoPerson {
            name: "Jojobell".to_string(),
            magnitude: 42.0,
        };
        let name_path = SimplePath::<String>::new(&[Field::Name("name")]);
        assert_eq!(person.item_at_path(&name_path).unwrap(), "Jojobell");
        person.name = "Colin".into();
        assert_eq!(person.item_at_path(&name_path).unwrap(), "Colin");
    }

    #[test]
    fn nested_keypath() {
        let person = DemoPerson {
            name: "coco".to_string(),
            magnitude: 42.0,
        };

        let person1 = DemoPerson {
            name: "jojo".to_string(),
            magnitude: 69.0,
        };

        let mut demo = DemoStruct {
            friends: vec![person, person1],
        };

        let jojo_name: SimplePath<String> =
            SimplePath::new(&[Field::Name("friends"), Field::Ord(1), Field::Name("name")]);

        assert_eq!(demo.item_at_path(&jojo_name).unwrap(), "jojo");
        demo.set_item_at_path(&jojo_name, "Brad".into()).unwrap();
        assert_eq!(demo.item_at_path(&jojo_name).unwrap(), "Brad");
        assert_eq!(demo.friends[1].name, "Brad");
    }
}
