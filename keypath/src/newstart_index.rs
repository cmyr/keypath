use crate::newstart::{KeyPath, Keyable};
use std::ops::Index;

use serde::de::{DeserializeOwned, IntoDeserializer};
use serde_json::{self, json, Value};

//impl<'a, V, T> Index<KeyPath<'a, V>> for T
//where T: ConcreteKeyable,
//V: DeserializeOwned,
//{
//type Output = Option<V>;
//fn index(&self, index: KeyPath<V>) -> Option<V> {
//self.get(index)
//}

//}

trait ConcreteKeyable {
    fn get<T: DeserializeOwned>(&self, key: KeyPath<T>) -> Option<T>;
}

impl ConcreteKeyable for Value {
    fn get<T: DeserializeOwned>(&self, key: KeyPath<T>) -> Option<T> {
        let val = self.value_at_path(key.as_slice()).ok()?;
        serde_json::from_value(val.to_owned()).ok()
    }
}

struct CachedKeyStore<T: ConcreteKeyable> {
    inner: T,
    cache: HashMap<String, ()>,
}

//impl<'de, U: IntoDeserializer<'de> + Keyable> ConcreteKeyable for U {
//fn get<T: DeserializeOwned>(&self, key: KeyPath<T>) -> Option<T> {
//let val = self.value_at_path(key.as_slice()).ok()?;
//Self::deserialize(val.into_deserializer()).ok()
//}
//}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

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

        let street_no: KeyPath<u32> = KeyPath::new("address.number").unwrap();
        //let sammat_floor: KeyPath<()> = KeyPath::new("rooms.0.floor").unwrap();
        //let no_exist: KeyPath<()> = KeyPath::new("rooms.hello.pals").unwrap();

        assert_eq!(recurse_center.get(street_no), Some(397));

        #[derive(Debug, Clone, Deserialize)]
        struct Room {
            name: String,
            capacity: u32,
            floor: u16,
        }

        let sammat_key: KeyPath<Room> = KeyPath::new("rooms.0").unwrap();
        let sammat = recurse_center.get(sammat_key).unwrap();
        assert_eq!(sammat.capacity, 16);
        //assert!(recurse_center.value_at_path(sammat_floor.as_slice()).is_ok());
        //assert!(recurse_center.value_at_path(no_exist.as_slice()).is_err());
    }
}
