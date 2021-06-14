#[macro_use]
extern crate serde_derive;
use std::any::Any;
use std::marker::PhantomData;

//mod lib2;
//mod json_keypath;
//mod lib3;
//mod newstart;
//mod newstart_index;
//mod value;

#[derive(Debug, Clone, Copy)]
struct SimplePath<T: 'static> {
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

trait RawKeyable: 'static {
    fn as_any(&self) -> &dyn Any;
    fn get_field(&self, ident: &[Field]) -> Option<&dyn RawKeyable>;
}

trait Keyable: RawKeyable {
    fn item_at_path<T>(&self, path: &SimplePath<T>) -> Option<&T> {
        self.get_field(path.fields)
            .and_then(|t| t.as_any().downcast_ref())
    }
}

macro_rules! keyable_leaf {
    ($name:ty) => {
        impl RawKeyable for $name {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn get_field(&self, ident: &[Field]) -> Option<&dyn RawKeyable> {
                if ident.is_empty() {
                    Some(self)
                } else {
                    None
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

    fn get_field(&self, ident: &[Field]) -> Option<&dyn RawKeyable> {
        match ident.split_first() {
            None => Some(self),
            Some((Field::Ord(idx), rest)) => self.get(*idx).and_then(|t| t.get_field(rest)),
            _ => None,
        }
    }
}

struct DemoStruct {
    friends: Vec<DemoPerson>,
}

struct DemoPerson {
    name: String,
    magnitude: f64,
}

impl RawKeyable for DemoPerson {
    fn get_field(&self, ident: &[Field]) -> Option<&dyn RawKeyable> {
        //let head, rest = ident.split_first
        match ident.split_first() {
            None => Some(self),
            Some((Field::Name("name"), rest)) => self.name.get_field(rest),
            Some((Field::Name("magnitude"), rest)) => self.magnitude.get_field(rest),
            _ => None,
        }
    }

    fn as_any(&self) -> &(dyn Any + 'static) {
        self
    }
}

impl Keyable for DemoPerson {}

impl RawKeyable for DemoStruct {
    fn get_field(&self, ident: &[Field]) -> Option<&dyn RawKeyable> {
        match ident.split_first() {
            None => Some(self),
            Some((Field::Name("friends"), rest)) => self.friends.get_field(rest),
            _ => None,
        }
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

        let demo = DemoStruct {
            friends: vec![person, person1],
        };

        let jojo_name: SimplePath<String> =
            SimplePath::new(&[Field::Name("friends"), Field::Ord(1), Field::Name("name")]);

        assert_eq!(demo.item_at_path(&jojo_name).unwrap(), "jojo");
    }
}
