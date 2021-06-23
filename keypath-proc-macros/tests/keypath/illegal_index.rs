use keypath::{keypath, Keyable};

#[derive(Keyable)]
struct DemoStruct {
    friend_lists: Vec<Person>,
}

#[derive(Keyable)]
struct Person {
    name: String,
    magnitude: f64,
}

fn main() {

    let _ = keypath!(DemoStruct.friend_lists[1.0].name);
    let _ = keypath!(DemoStruct.friend_lists[-5].name);
    let _ = keypath!(DemoStruct.friend_lists[5_u8].name);
    let _ = keypath!(DemoStruct.friend_lists[5_f64].name);
    let _ = keypath!(DemoStruct.friend_lists[five].name);
    let _ = keypath!(DemoStruct.friend_lists['5'].name);

}
