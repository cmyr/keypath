use keypath::{keypath, Keyable, TypedKeyable};

#[derive(Keyable)]
pub struct DemoPerson {
    name: String,
    magnitude: f64,
}

#[test]
fn simple_keypath() {
    let mut person = DemoPerson {
        name: "Jojobell".to_string(),
        magnitude: 42.0,
    };
    let name_path = keypath!(DemoPerson.name);
    assert_eq!(person.item_at_path(&name_path), "Jojobell");
    person.name = "Colin".into();
    assert_eq!(person.item_at_path(&name_path), "Colin");
}
