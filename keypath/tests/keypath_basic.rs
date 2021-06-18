use keypath::{keypath, Keyable, TypedKeyable};

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
fn keypath() {
    let mut person = DemoPerson {
        name: "coco".to_string(),
        magnitude: 42.0,
        size: Size {
            big: true,
            heft: 200,
        },
    };

    let path = keypath!(DemoPerson.size.heft);
    assert_eq!(person[&path], 200);
    person[&path] = 15;
    assert_eq!(person.size.heft, 15);
}
