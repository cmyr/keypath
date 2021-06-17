use keypath::{keypath, Keyable, TypedKeyable};

#[derive(Keyable)]
pub struct DemoStruct {
    friends: Vec<DemoPerson>,
}

#[derive(Keyable)]
pub struct DemoPerson {
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
    let person = DemoPerson {
        name: "coco".to_string(),
        magnitude: 42.0,
        size: Size {
            big: true,
            heft: 200,
        },
    };

    let person1 = DemoPerson {
        name: "jojo".to_string(),
        magnitude: 69.0,
        size: Size {
            big: true,
            heft: 200,
        },
    };

    let mut demo = DemoStruct {
        friends: vec![person, person1],
    };

    let jojo_name = keypath!(DemoStruct.friends[1].name);

    assert_eq!(demo.item_at_path(&jojo_name), "jojo");
    demo.set_item_at_path(&jojo_name, "Brad".into());
    assert_eq!(demo.item_at_path(&jojo_name), "Brad");
    assert_eq!(demo.friends[1].name, "Brad");
}
