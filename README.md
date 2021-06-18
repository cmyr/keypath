# `keypath`

*Strongly typed references to arbitrarily nested fields.*

This is an early experiment in implementing Swift-style [keypaths] in Rust. It
is currently intended as a proof of concept, and is missing some fancier features
such as 'partial keypaths' and composibility, although implementing these should
not be especially challanging. What this *does* include is what I believe is the
most difficult case, of generating typed keypaths for arbitrary types that are
guaranteed at compile time.

This means you can do the following:

```rust
#[derive(Keyable)]
struct Person {
    name: String,
    friends: Vec<String>,
    size: Size,
}
#[derive(Keyable)]
struct Size {
    big: bool,
    heft: u8,
}

let mut person = Person {
    name: "coco".into(),
    friends: vec!["eli".into(), "nico".into(), "yaya".into()],
    size: Size { big: false, heft: 45 }
};

let first_friend: KeyPath<Person, String> = keypath!(Person.friends[0]);
let heft = keypath!(Person.size.heft);

assert_eq!(person[&first_friend], "eli");

// mutation:
person[&heft] = 101;
assert_eq!(person.size.heft, 101);
```

This may not seem especially useful on its own, but it is an ergonomic building
block for things like UI bindings, and observable objects.


[keypaths]: https://www.swiftbysundell.com/articles/the-power-of-key-paths-in-swift/
