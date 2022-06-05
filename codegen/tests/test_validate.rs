#[test]
fn test_func() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/func/no_metadata.rs");
    t.compile_fail("tests/ui/func/no_url.rs");
    t.compile_fail("tests/ui/func/no_url2.rs");
    t.compile_fail("tests/ui/func/invalid_url.rs");
    t.compile_fail("tests/ui/func/async.rs");
    t.compile_fail("tests/ui/func/return_value.rs");
    t.compile_fail("tests/ui/func/return_value2.rs");
    t.compile_fail("tests/ui/func/attr.rs");
    t.compile_fail("tests/ui/func/attr2.rs");
    t.compile_fail("tests/ui/func/param.rs");
    t.compile_fail("tests/ui/func/form.rs");
    t.compile_fail("tests/ui/func/form2.rs");
    t.compile_fail("tests/ui/func/body.rs");
    t.compile_fail("tests/ui/func/body_form.rs");
}

#[test]
fn test_struct() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/struct/no_metadata.rs");
    t.compile_fail("tests/ui/struct/no_url.rs");
    t.compile_fail("tests/ui/struct/no_url2.rs");
    t.compile_fail("tests/ui/struct/method.rs");
    t.compile_fail("tests/ui/struct/path.rs");
}
