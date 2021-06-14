#[macro_use]
extern crate serde_derive;
use std::any::Any;
use std::borrow::Cow;
use std::marker::PhantomData;

//mod lib2;
//mod json_keypath;
//mod lib3;
//mod newstart;
//mod newstart_index;
//mod value;

#[derive(Debug, Clone, Copy)]
pub enum Field {
    Ord(usize),
    Name(&'static str),
    //Identity,
}

//pub struct KeyPath<T, const N: usize> {
//path: [Field; N],
//_type: PhantomData<T>,
//}

//pub struct KeyPath<T> {
//path: &'static [Field],
//_type: PhantomData<T>,
//}

//enum KeyPathError {}

//pub trait Keyable {
//fn get_item_at_path<T>(&self, path: KeyPath<T>) -> Result<&T, KeyPathError>;
////fn get_item_as_str(&self)
//}

struct DemoStruct {
    friends: Vec<DemoPerson>,
}

struct DemoPerson {
    name: String,
    magnitude: f64,
}

macro_rules! keyable_leaf {
    ($name:ty) => {
        impl RawKeyable for $name {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn get_field(&self, _ident: &Field) -> Option<&dyn RawKeyable> {
                None
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

    fn get_field(&self, ident: &Field) -> Option<&dyn RawKeyable> {
        match ident {
            Field::Ord(idx) => self.get(*idx).map(|t| t as &dyn RawKeyable),
            _ => None,
        }
    }

    //fn item_at_path<T>(&self, path: &SimplePath<T>) -> Option<&T> {
    //self._keypath_get_value(path.name)
    //.and_then(<dyn Any>::downcast_ref)
    //}
}

trait RawKeyable: 'static {
    fn as_any(&self) -> &dyn Any;
    fn get_field(&self, ident: &Field) -> Option<&dyn RawKeyable>;
}

//trait Keyable: RawKeyable {
//fn item_at_path<T>(&self, path: &SimplePath<T>) -> Option<&T> {
////self.get_field(path.name)
////.and_then(<dyn Any>::downcast_ref)
//}
//}

impl RawKeyable for DemoPerson {
    fn get_field(&self, ident: &Field) -> Option<&dyn RawKeyable> {
        match ident {
            Field::Name("name") => Some(&self.name),
            Field::Name("magnitude") => Some(&self.magnitude),
            _ => None,
        }
    }

    fn as_any(&self) -> &(dyn Any + 'static) {
        self
    }
}

impl RawKeyable for DemoStruct {
    fn get_field(&self, ident: &Field) -> Option<&dyn RawKeyable> {
        match ident {
            Field::Name("friends") => Some(&self.friends),
            _ => None,
        }
    }

    fn as_any(&self) -> &(dyn Any + 'static) {
        self
    }
}

//impl DemoPerson {
//fn item_at_path<T>(&self, path: &SimplePath<T>) -> Option<&T> {
//self._keypath_get_value(path.name)
//.and_then(<dyn Any>::downcast_ref)
//}
//}

//impl DemoStruct {
//fn _keypath_get_value(&self, ident: &str) -> Option<&dyn Any> {
//match ident {
//"friends" => Some(&self.friends),
////"magnitude" => Some(&self.magnitude),
//_ => None,
//}
//}

//fn item_at_path<T>(&self, path: &SimplePath<T>) -> Option<&T> {
//self._keypath_get_value(path.name)
//.and_then(<dyn Any>::downcast_ref)
//}
//}

#[derive(Debug, Clone, Copy)]
struct SimplePath<T: 'static> {
    //name: &'static str,
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

//struct ConcreteKeyPath<Root, Value> {
//_root: PhantomData<Root>,

//}

fn hi() {
    //let path = keypath!(DemoStruct, .friends[0].name);
}

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
}
