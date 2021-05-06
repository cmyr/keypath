use crate::value::{AsValue, Error, TryFromValue, Value};
use serde::de::IntoDeserializer;

#[derive(Default)]
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

impl BufferItems {
    fn value_for_key<'a, T: TryFromValue<'a>>(&'a self, key: &str) -> Result<T, Error> {
        match key {
            "line_ending" => self.line_ending.as_value().try_as_type(),
            "tab_size" => self.tab_size.as_value().try_as_type(),
            "translate_tabs_to_spaces" => self.translate_tabs_to_spaces.as_value().try_as_type(),
            "use_tab_stops" => self.use_tab_stops.as_value().try_as_type(),
            "font_face" => self.font_face.as_value().try_as_type(),
            "font_size" => self.font_size.as_value().try_as_type(),
            "auto_indent" => self.auto_indent.as_value().try_as_type(),
            "scroll_past_end" => self.scroll_past_end.as_value().try_as_type(),
            "wrap_width" => self.wrap_width.as_value().try_as_type(),
            "word_wrap" => self.word_wrap.as_value().try_as_type(),
            "autodetect_whitespace" => self.autodetect_whitespace.as_value().try_as_type(),
            "surrounding_pairs" => Err("sowwy i'm a vec"),
            _other => Err("unknown key"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn something() {
        let mut items = BufferItems::default();
        items.use_tab_stops = true;
        items.wrap_width = 55;
        items.font_face = String::from("Rofls Sans");

        assert_eq!(items.value_for_key::<usize>("fake key"), Err("unknown key"));
        assert_eq!(items.value_for_key("word_wrap"), Ok(false));
        assert_eq!(items.value_for_key("wrap_width"), Ok(55_usize));
    }
}
