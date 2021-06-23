use keypath::{keypath, Keyable};

#[derive(Keyable)]
pub struct TuplePerson(String, f64, Size);

#[derive(Keyable)]
pub struct Size(f64, f64);

fn main() {
    let _ = keypath!(TuplePerson.2. 0);
}
