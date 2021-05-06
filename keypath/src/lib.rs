#[macro_use]
extern crate serde_derive;
use std::borrow::Cow;
use std::marker::PhantomData;

//mod lib2;
mod json_keypath;
mod lib3;
mod newstart;
mod newstart_index;
mod value;

//use serde::de::{Deserialize, IntoDeserializer};

//trait Keyable<'a>: Deserialize<'a> + IntoDeserializer<'a> {
//const TYPE: KeyableType;
//fn value_at_path<Val: Keyable<'a>>(&self, path: KeySlice<Val>) -> Result<&Val, ()> {
//match (Self::TYPE, path.component()) {
//(KeyableType::Map, KeyPathComponent::Member(key)) => self.get_keypath_key(key),
//(KeyableType::List, KeyPathComponent::Index(idx)) => self.get_keypath_index(*idx),
//(KeyableType::Value, KeyPathComponent::Terminal) => {
//Val::deserialize(self.into_deserializer())
//}
//}
//}

//fn value_at_path2<Val: Keyable<'a>>(&self, path: KeySlice<Val>) -> Result<&Val, ()> {
//let mut node: &dyn Keyable = &self;
//for key in path.components() {
//node = node.get_child(key);
//}
//}

//fn get_child(&self, key: KeyPathComponent<'a>) -> Option<&dyn Keyable> {
//unimplemented!()
//}

//fn get_keypath_key<T: Keyable<'a>>(&self, key: &str) -> Result<&T, ()>;
//fn get_keypath_index<T: Keyable<'a>>(&self, key: usize) -> Result<&T, ()>;
//}

//// impl sketch
//enum KeyableType {
//Map,
//Value,
//List,
//}

//enum Key<'a> {
//Index(usize),
//Name(&'a str),
//}

//enum KeyPathComponent<'a> {
//Terminal,
//Member(&'a str),
//Index(usize),
//}

//struct KeyPath<'a, Val> {
//raw: Cow<'a, str>,
//els: Vec<KeyPathComponent<'a>>,
//value: PhantomData<Val>,
//}

//struct KeySlice<'a, Val> {
//path: &'a [KeyPathComponent<'a>],
//value: PhantomData<Val>,
//}

//impl<'a, Val> KeySlice<'a, Val> {
//fn components(&self) -> PathComponents<'a> {
//PathComponents {  }
//}

//fn component(&self) -> &KeyPathComponent {
//self.path.first().unwrap()
//}
//}

//struct PathComponents<'a> {

//}

//impl<'a> Iterator for PathComponents<'a> {
//type Item=KeyPathComponent<'a>;
//fn next(&mut self) -> Option<Self::Item> {
//None
//}
//}
