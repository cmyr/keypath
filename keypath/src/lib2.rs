use std::borrow::Cow;
use std::marker::PhantomData;

trait Keyable {
    fn value_at_path<Val>(&self, path: KeySlice<Val>) -> Result<&Val, ()>;
    fn get_child<K: Keyable>(&self, key: &Key) -> Result<&K, ()>;
    fn get_value<Val>(&self, key: &Key) -> Result<&Val, ()>;
}


struct BufferItems {
    pub line_ending: String,
    pub tab_size: usize,
    pub translate_tabs_to_spaces: bool,
    pub use_tab_stops: bool,
    pub font_face: String,
    pub font_size: f32,
    pub auto_indent: bool,
    pub scroll_past_end: bool,
    pub wrap_width: usize,
    pub word_wrap: bool,
    pub autodetect_whitespace: bool,
    pub surrounding_pairs: Vec<(String, String)>,
}

impl Keyable for BufferItems {
    fn value_at_path<Val>(&self, path: KeySlice<Val>) -> Result<&Val, ()> {
        let (next_key, rest) = path.split_next();
        match rest {
            Some(rest) => self.get_child(next_key).map(|c| c.value_at_path(rest)),
            None => self.get_value(next_key),
        }
    }

    fn get_child<K: Keyable>(&self, key: &Key) -> Result<&K, ()> { Err(()) }
    fn get_value<Val>(&self, key: &Key) -> Result<&Val, ()> { Err(()) }
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


