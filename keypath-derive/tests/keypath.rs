use keypath::{keypath, Keyable};

#[derive(Keyable)]
struct DemoStruct {
    friends: Vec<DemoPerson>,
}

#[derive(Keyable)]
struct DemoPerson {
    name: String,
    magnitude: f64,
}

#[test]
fn smoke_test() {
    let path = keypath!(DemoStruct.0.name);
}
