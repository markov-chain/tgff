extern crate tgff;
extern crate test;

use std::io::fs::PathExtensions;
use std::io::File;

#[bench]
fn parser_process_simple(b: &mut test::Bencher) {
    let content = read_fixture("simple.tgff");

    b.iter(|| {
        test::black_box(tgff::parse(content.as_slice()).unwrap())
    });
}

#[bench]
fn parser_process_032_640(b: &mut test::Bencher) {
    let content = read_fixture("032_640.tgff");

    b.iter(|| {
        test::black_box(tgff::parse(content.as_slice()).unwrap())
    });
}

fn read_fixture(name: &'static str) -> String {
    let path = Path::new("fixtures").join(name);
    assert!(path.exists());
    File::open(&path).read_to_string().unwrap()
}
