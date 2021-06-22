use keypath::{keypath, KeyPath, Keyable};

const NAME_PATH: KeyPath<DemoPerson, String> = keypath!(DemoPerson.name);

#[derive(Keyable)]
struct DemoPerson {
    name: String,
    magnitude: f64,
}

#[test]
fn simple_keypath() {
    let mut person = DemoPerson {
        name: "Jojobell".to_string(),
        magnitude: 42.0,
    };

    assert_eq!(person[&NAME_PATH], "Jojobell");
    person.name = "Colin".into();
    assert_eq!(person[&NAME_PATH], "Colin");
    person[&NAME_PATH] = "Sriti".into();
    assert_eq!(person[&NAME_PATH], "Sriti");
}
