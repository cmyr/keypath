use std::borrow::Cow;
use std::marker::PhantomData;
use crate::value::{Value, AsValue, TryFromValue, Error as ValueError};

enum Error {
    Value(ValueError),
}

trait Keyable<'a> {
    fn value_at_path<Val: TryFromValue<'a>>(&'a self, path: KeySlice<Val>) -> Result<Val, ()>;
    fn get_child<K: Keyable<'a>>(&'a self, key: &Key) -> Result<&K, ()>;
    fn get_value<Val: TryFromValue<'a>>(&'a self, key: &Key) -> Result<Val, ()>;
}


#[derive(Default)]
struct Things {
    truth: bool,
    count: usize,
    sound: String,
}

#[derive(Default)]
struct BufferItems {
    pub line_ending: String,
    pub tab_size: usize,
    pub translate_tabs_to_spaces: bool,
    pub use_tab_stops: bool,
    pub font_face: String,
    pub font_size: f32,
    pub truth: Things,
}

enum Key<'a> {
    Ord(usize),
    Name(&'a str),
}

struct KeySlice<'a, Val> {
    path: &'a [Key<'a>],
    value: PhantomData<Val>,
}

impl<'a, Val> KeySlice<'a, Val> {
    fn split_next(&self) -> (&'a Key<'a>, Option<KeySlice<'a, Val>>) {
        let next = self.path[0];
        let rest = if self.path.len() == 1 {
            None
        } else {
            let path = self.path[1..];
            Some(KeySlice {
                path,
                value: PhantomData,
            })
        };
    }
}

impl<'a> Keyable<'a> for BufferItems {
    fn value_at_path<Val>(&self, path: KeySlice<Val>) -> Result<&Val, ()> {
        let (next_key, rest) = path.split_next();
        match (next_key, rest) {
            (next_key, Some(rest)) => self.get_child(next_key)?.value_at_path(rest),
            (next_key, None) => self.get_value(next_key),
        }
    }

    fn get_child<K: Keyable>(&self, key: &Key) -> Result<&K, ()> { 

    }
    fn get_value<Val>(&self, key: &Key) -> Result<&Val, ()> { Err(()) }
}

