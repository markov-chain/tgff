extern crate tgff;
extern crate test;

use std::io::fs::PathExtensions;
use std::io::File;
use tgff::Parser;

#[bench]
fn parser_process_simple(b: &mut test::Bencher) {
    let content = read_fixture("simple.tgff");

    b.iter(|| {
        let mut parser = Parser::new(content.as_slice());
        test::black_box(parser.process().unwrap())
    });
}

#[bench]
fn parser_process_032_640(b: &mut test::Bencher) {
    let content = read_fixture("032_640.tgff");

    b.iter(|| {
        let mut parser = Parser::new(content.as_slice());
        test::black_box(parser.process().unwrap())
    });
}

fn read_fixture(name: &'static str) -> String {
    let path = Path::new("tests").join_many(["fixtures", name]);
    assert!(path.exists());
    File::open(&path).read_to_string().unwrap()
}
