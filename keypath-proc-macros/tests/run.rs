#[test]
fn keypath() {
    let t = trybuild::TestCases::new();
    t.pass("tests/keypath/basic_structs.rs");
    t.compile_fail("tests/keypath/illegal_spaces.rs");
    t.compile_fail("tests/keypath/illegal_period.rs");
    t.compile_fail("tests/keypath/illegal_index.rs");
}
