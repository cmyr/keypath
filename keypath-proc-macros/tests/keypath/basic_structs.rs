use keypath::{keypath, Keyable};

#[derive(Keyable)]
pub struct TuplePerson(String, f64, Size);

#[derive(Keyable)]
pub struct Size(f64, f64);

#[derive(Keyable)]
struct Person {
    name: String,
    stats: Stats,
}

#[derive(Keyable)]
struct Stats {
    width: f32,
    breadth: bool,
}


fn main() {
    let _ = keypath!(TuplePerson.2.0);
    let _ = keypath!(Person.stats.width);
}
