
trait Plugin {
    const &str NAME;
    fn manifest() -> &'static Manifest;
    fn start(info: LaunchInfo) -> Result<Self, LaunchError>;
    fn handle_event(&mut self, ctx: &Context, event: PluginEvent);
    fn terminate(&mut self);
}


let plugins: Vec<Box<dyn Plugin>>;


trait Keyable: Deserialize {
    const TYPE: KeyableType;
    fn value_at_path<Val>(&self, path: KeySlice<Val>) -> Result<Val, ()>;
}

// impl sketch
fn value_at_path<Val>(&self, path: KeySlice<Val>) -> Result<Val, ()> {
    match (Self::TYPE, path.component()) {
        (KeyableType::Map, KeyPathComponent::Member(key)) => 
            self.get(key),
        (KeyableType::List, KeyPathComponent::Index(idx)) =>
            self.get_index(idx),
        (KeyableType::Value, KeyPathComponent::Terminal) =>
            Val::deserialize(self.into_deserializer())
    }
}   

enum KeyableType {
    Map,
    Value,
    List,
}

enum KeyPathComponent<'a> {
    Terminal,
    Member(&'a str),
    Index(usize),
}

struct KeyPath<'a, Val> {
    raw: Cow<str, 'a>,
    els: Vec<&'a str>,
    value: PhantomData<Val>,
}

struct KeySlice<'a, Val> {
    path: &'a [&'a str],
    value: PhantomData<Val>,
}
    
