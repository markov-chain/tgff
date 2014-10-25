extern crate test;

extern crate tgff;

#[bench]
fn parse_002_040(b: &mut test::Bencher) {
    let content = read_fixture("002_040.tgff");

    b.iter(|| {
        test::black_box(tgff::parse(content.as_slice()).unwrap())
    });
}

#[bench]
fn parse_032_640(b: &mut test::Bencher) {
    let content = read_fixture("032_640.tgff");

    b.iter(|| {
        test::black_box(tgff::parse(content.as_slice()).unwrap())
    });
}

fn read_fixture(name: &str) -> String {
    use std::io::fs::PathExtensions;
    let path = Path::new("tests").join_many(["fixtures", name]);
    assert!(path.exists());
    std::io::File::open(&path).read_to_string().unwrap()
}
