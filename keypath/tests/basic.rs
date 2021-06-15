use keypath::{Field, Keyable, SimplePath};

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
fn simple_keypath() {
    let mut person = DemoPerson {
        name: "Jojobell".to_string(),
        magnitude: 42.0,
    };
    let name_path = SimplePath::<String>::new(&[Field::Name("name")]);
    assert_eq!(person.item_at_path(&name_path).unwrap(), "Jojobell");
    person.name = "Colin".into();
    assert_eq!(person.item_at_path(&name_path).unwrap(), "Colin");
}

#[test]
fn nested_keypath() {
    let person = DemoPerson {
        name: "coco".to_string(),
        magnitude: 42.0,
    };

    let person1 = DemoPerson {
        name: "jojo".to_string(),
        magnitude: 69.0,
    };

    let mut demo = DemoStruct {
        friends: vec![person, person1],
    };

    let jojo_name: SimplePath<String> =
        SimplePath::new(&[Field::Name("friends"), Field::Ord(1), Field::Name("name")]);

    assert_eq!(demo.item_at_path(&jojo_name).unwrap(), "jojo");
    demo.set_item_at_path(&jojo_name, "Brad".into()).unwrap();
    assert_eq!(demo.item_at_path(&jojo_name).unwrap(), "Brad");
    assert_eq!(demo.friends[1].name, "Brad");
}
