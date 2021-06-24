use keypath::{keypath, Keyable};

#[derive(Keyable)]
struct Generic<T> {
    name: String,
    value: T,
}

#[test]
fn simple_keypath() {
    let mut person = Generic {
        name: "Jojobell".to_string(),
        value: true,
    };

    let name_path = keypath!(Generic<bool>.name);
    let value_path = keypath!(Generic<bool>.value);
    assert_eq!(person[&name_path], "Jojobell");
    person.name = "Colin".into();
    assert_eq!(person[&name_path], "Colin");
    person[&name_path] = "Sriti".into();
    assert_eq!(person[&name_path], "Sriti");
    person[&value_path] = false;
    assert!(!person.value);
}
