#![feature(macro_rules)]

extern crate tgff;
extern crate test;

use std::io::fs::PathExtensions;
use std::io::File;
use tgff::Parser;

macro_rules! assert_ok(
    ($e: expr) => (
        if let Err(err) = $e {
            assert!(false, "{}", err);
        }
    );
)

#[test]
fn parser_process() {
    let content = read_fixture("simple.tgff");
    let mut parser = Parser::new(content.as_slice());
    assert_ok!(parser.process());
}

fn read_fixture(name: &'static str) -> String {
    let path = Path::new("tests").join_many(["fixtures", name]);
    assert!(path.exists());
    File::open(&path).read_to_string().unwrap()
}
