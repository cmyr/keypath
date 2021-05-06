use std::marker::PhantomData;
use std::ops::Index;

use serde::de::DeserializeOwned;
use serde_json::{self, json, Value};

// some host type
// json, toml, etc
//
// 1. traverse graph to find value
// 2. transform value to expected type

// a keypath is something like "hello.friends.0.name"

pub trait Keyable: DeserializeOwned {
    fn value_at_path(&self, path: KeySlice) -> Result<&Self, ()>;
}

impl Keyable for Value {
    fn value_at_path(&self, path: KeySlice) -> Result<&Self, ()> {
        let (next_key, rest) = path.split_next();
        let child = match next_key {
            KeyPart::Ord(idx) => self.get(idx).ok_or(())?,
            KeyPart::Name(name) => self.get(name).ok_or(())?,
        };

        match rest {
            Some(more_path) => child.value_at_path(more_path),
            None => Ok(child),
        }
    }
}

pub struct KeyPath<'a, Val> {
    raw: &'a str,
    els: Vec<KeyPart<'a>>,
    value: PhantomData<Val>,
}

pub struct KeySlice<'a> {
    path: &'a [KeyPart<'a>],
    //value: PhantomData<Val>,
}

impl<'a, Val> KeyPath<'a, Val> {
    pub fn new(raw: &'a str) -> Result<Self, ()> {
        let els: Result<Vec<KeyPart>, ()> = raw.split('.').map(KeyPart::from_raw).collect();
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

    pub fn as_slice(&'a self) -> KeySlice<'a> {
        KeySlice {
            path: self.els.as_slice(),
            //value: PhantomData,
        }
    }
}

impl<'a> KeySlice<'a> {
    fn split_next(&self) -> (&'a KeyPart<'a>, Option<KeySlice<'a>>) {
        let next = &self.path[0];
        let rest = if self.path.len() == 1 {
            None
        } else {
            let path = &self.path[1..];
            Some(KeySlice {
                path,
                //value: PhantomData,
            })
        };
        (next, rest)
    }

    //fn transmute<T>(self) -> KeySlice<'a, T> {
    //KeySlice {
    //path: self.path,
    //value: PhantomData
    //}
    //}
}

#[derive(Debug, PartialEq)]
enum KeyPart<'a> {
    Ord(usize),
    Name(&'a str),
}

impl<'a> KeyPart<'a> {
    fn from_raw(src: &'a str) -> Result<Self, ()> {
        if src.is_empty() {
            return Err(());
        }

        let result = match src.as_bytes()[0] {
            b'0'...b'9' => KeyPart::Ord(src.parse().map_err(|_| ())?),
            other => KeyPart::Name(src),
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smoke_test() {
        let recurse_center = json!({
            "address": {
                "street": "bridge",
                "number": 397,
            },
            "rooms": [
            {
                "name": "sammat",
                "capacity": 16,
                "floor": 4,
            },
            {
                "name": "djikstra",
                "capacity": 4,
                "floor": 5,
            }
            ],
        });

        let street_no: KeyPath<()> = KeyPath::new("address.number").unwrap();
        let sammat_floor: KeyPath<()> = KeyPath::new("rooms.0.floor").unwrap();
        let no_exist: KeyPath<()> = KeyPath::new("rooms.hello.pals").unwrap();

        assert!(recurse_center.value_at_path(street_no.as_slice()).is_ok());
        assert!(recurse_center
            .value_at_path(sammat_floor.as_slice())
            .is_ok());
        assert!(recurse_center.value_at_path(no_exist.as_slice()).is_err());
    }
}
