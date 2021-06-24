#[test]
fn keypath() {
    let t = trybuild::TestCases::new();
    t.pass("tests/keypath/basic_structs.rs");
    t.compile_fail("tests/keypath/invalid_path_syntax.rs");
    t.pass("tests/keypath/generic.rs");
    t.compile_fail("tests/keypath/generic_const_fail.rs");
}
