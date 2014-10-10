extern crate tgff;
extern crate test;

use std::io::fs::PathExtensions;

#[test]
fn parse() {
    assert!(tgff::parse(find_fixture("simple.tgff")).is_ok());
}

fn find_fixture(name: &'static str) -> Path {
    let path = Path::new("tests").join_many(["fixtures", name]);
    assert!(path.exists());
    path
}
