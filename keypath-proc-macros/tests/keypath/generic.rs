use keypath::{keypath, Keyable};

#[derive(Keyable)]
struct Container<T> {
    names: Vec<T>,
}

fn main() {
    let _ = keypath!(Container<String>.names[0]);
}
