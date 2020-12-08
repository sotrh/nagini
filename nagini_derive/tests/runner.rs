#[test]
fn run_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/parse.rs"); // TODO: figure how to get trybuild to include the correct file
}