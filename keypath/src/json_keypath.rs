use serde::de::DeserializeOwned;
use serde_json::{self, json, Value};
use std::marker::PhantomData;

trait Keyable {
    fn value_at_path(&self, path: KeySlice<Value>) -> Result<&Value, ()>;
}

trait Keyable2 {
    fn value_at_path2<T: DeserializeOwned>(&self, path: KeySlice<T>) -> Result<T, ()>;
}

impl Keyable for Value {
    fn value_at_path(&self, path: KeySlice<Value>) -> Result<&Value, ()> {
        let (next_key, rest) = path.split_next();
        let child = match next_key {
            Key::Ord(idx) => self.get(idx).ok_or(())?,
            Key::Name(name) => self.get(name).ok_or(())?,
        };

        match rest {
            Some(more_path) => child.value_at_path(more_path),
            None => Ok(child),
        }
    }
}

impl Keyable2 for Value {
    fn value_at_path2<T: DeserializeOwned>(&self, path: KeySlice<T>) -> Result<T, ()> {
        let val_slice = path.transmute();
        let child = self.value_at_path(val_slice)?;
        serde_json::from_value(child.to_owned()).map_err(|_| ())
    }
}

#[derive(Debug, PartialEq)]
enum Key<'a> {
    Ord(usize),
    Name(&'a str),
}

impl<'a> Key<'a> {
    fn from_raw(src: &'a str) -> Result<Self, ()> {
        if src.is_empty() {
            return Err(());
        }

        let result = match src.as_bytes()[0] {
            b'0'...b'9' => Key::Ord(src.parse().map_err(|_| ())?),
            other => Key::Name(src),
        };
        Ok(result)
    }
}

struct KeySlice<'a, Val> {
    path: &'a [Key<'a>],
    value: PhantomData<Val>,
}

struct KeyPath<'a, Val> {
    raw: &'a str,
    els: Vec<Key<'a>>,
    value: PhantomData<Val>,
}

impl<'a, Val> KeyPath<'a, Val> {
    fn new(raw: &'a str) -> Result<Self, ()> {
        let els: Result<Vec<Key>, ()> = raw.split('.').map(Key::from_raw).collect();
        let els = els?;
        if els.is_empty() {
            return Err(());
        }

        Ok(KeyPath {
            raw,
            els,
            value: PhantomData,
        })
    }

    fn as_slice(&'a self) -> KeySlice<'a, Val> {
        KeySlice {
            path: self.els.as_slice(),
            value: PhantomData,
        }
    }
}

impl<'a, Val> KeySlice<'a, Val> {
    fn split_next(&self) -> (&'a Key<'a>, Option<KeySlice<'a, Val>>) {
        let next = &self.path[0];
        let rest = if self.path.len() == 1 {
            None
        } else {
            let path = &self.path[1..];
            Some(KeySlice {
                path,
                value: PhantomData,
            })
        };
        (next, rest)
    }

    fn transmute<T>(self) -> KeySlice<'a, T> {
        KeySlice {
            path: self.path,
            value: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test_keypath() {
        let raw = "hello.world.5.friend";
        let kp: KeyPath<Value> = KeyPath::new(&raw).unwrap();
        assert_eq!(
            kp.els,
            vec![
                Key::Name("hello"),
                Key::Name("world"),
                Key::Ord(5),
                Key::Name("friend")
            ]
        );
    }

    #[test]
    fn json_traversal() {
        let my_json = json!({
            "hello": {
                "bob": 5,
                "jeff": 8,
                "smriti": {
                    "hometown": "nyc",
                    "sworn enemy": true,
                }
            }
        });

        let kp = KeyPath::new("hello.smriti.hometown").unwrap();
        assert_eq!(my_json.value_at_path(kp.as_slice()), Ok(&json!("nyc")));
        let kp = KeyPath::new("hello.smriti.hometown.nyc").unwrap();
        assert_eq!(my_json.value_at_path(kp.as_slice()), Err(()));
    }

    #[test]
    fn typed_get_value() {
        let my_json = json!({
            "hello": {
                "bob": 5,
                "jeff": 8,
                "smriti": {
                    "hometown": "nyc",
                    "sworn enemy": true,
                }
            }
        });

        let kp: KeyPath<String> = KeyPath::new("hello.smriti.hometown").unwrap();
        let hometown = my_json.value_at_path2(kp.as_slice()).unwrap();
        assert_eq!(hometown, String::from("nyc"));
    }
}
