use keypath::{keypath, Keyable};

#[derive(Keyable)]
pub struct DemoPerson(String, f64, Size, (f32, f32));

#[derive(Keyable)]
pub struct Size(f64, f64);

#[test]
fn keypath() {
    let mut person = DemoPerson("coco".to_string(), 42.0, Size(1.0, 5.0), (2.0, 4.0));

    let path = keypath!(DemoPerson.2 .0);
    assert_eq!(person[&path], 1.0);
    person[&path] = 15.0;
    assert_eq!(person.2 .0, 15.0);
}
