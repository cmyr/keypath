use keypath::{keypath, Keyable, TypedKeyable};
use std::collections::HashMap;

#[derive(Keyable)]
pub struct DemoStruct {
    friend_lists: HashMap<String, Vec<Person>>,
}

#[derive(Keyable)]
pub struct Person {
    name: String,
    magnitude: f64,
    size: Size,
}

#[derive(Keyable)]
pub struct Size {
    big: bool,
    heft: u8,
}

#[test]
fn nested_keypath() {
    let coco = Person {
        name: "coco".to_string(),
        magnitude: 42.0,
        size: Size {
            big: true,
            heft: 200,
        },
    };

    let jojo = Person {
        name: "jojo".to_string(),
        magnitude: 69.0,
        size: Size {
            big: true,
            heft: 200,
        },
    };

    let mut friend_lists = HashMap::new();
    friend_lists.insert("work".to_string(), vec![coco]);
    friend_lists.insert("play".to_string(), vec![jojo]);

    let mut demo = DemoStruct { friend_lists };

    let jojo_name = keypath!(DemoStruct.friend_lists["play"][0].name);

    assert_eq!(demo.item_at_path(&jojo_name), "jojo");
    demo.set_item_at_path(&jojo_name, "Brad".into());
    assert_eq!(demo.item_at_path(&jojo_name), "Brad");
    assert_eq!(demo.friend_lists["play"][0].name, "Brad");
}

//#[test]
//fn some_error_meessages_to_improve() {
//// dot followed by brace
//let jojo_name = keypath!(DemoStruct.["play"][0].name);
//// two dots
//let jojo_name = keypath!(DemoStruct..["play"][0].name);
//}
