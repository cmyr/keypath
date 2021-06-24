use keypath::{keypath, KeyPath, Keyable};

#[derive(Keyable)]
struct Container<T> {
    names: Vec<T>,
}

const _PATH: KeyPath<Container<String>, String> = keypath!(Container<String>.names[0]);

fn main() {
}
