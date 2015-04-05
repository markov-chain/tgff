#![feature(test)]

extern crate test;

extern crate tgff;

#[bench]
fn parse_002_040(b: &mut test::Bencher) {
    let content = read_fixture("002_040.tgff");

    b.iter(|| {
        test::black_box(tgff::parse(&content).unwrap())
    });
}

#[bench]
fn parse_032_640(b: &mut test::Bencher) {
    let content = read_fixture("032_640.tgff");

    b.iter(|| {
        test::black_box(tgff::parse(&content).unwrap())
    });
}

fn read_fixture(name: &str) -> String {
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    let mut path = PathBuf::from("tests");
    path.push("fixtures");
    path.push(name);

    let mut buffer = String::new();
    File::open(&path).unwrap().read_to_string(&mut buffer).unwrap();

    buffer
}
