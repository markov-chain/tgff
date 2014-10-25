#![feature(phase)]

#[phase(plugin)] extern crate assert;
extern crate test;

extern crate tgff;

#[test]
fn parse_simple() {
    let r = tgff::parse(read_fixture("simple.tgff").as_slice()).unwrap();

    assert_eq!(r.attributes["HYPERPERIOD".to_string()], 1180);
    assert_eq!(r.graphs.len(), 5);
    assert_eq!(r.tables.len(), 3);

    let periods = vec![590, 1180, 1180, 590, 1180];
    let tasks = vec![12, 20, 24, 8, 20];
    let arcs = vec![19, 25, 28, 7, 24];
    let deadlines = vec![1, 6, 8, 3, 6];

    for i in range(0u, 5) {
        assert_eq!(r.graphs[i].name, "TASK_GRAPH".to_string());
        assert_eq!(r.graphs[i].id, i);
        assert_eq!(r.graphs[i].attributes["PERIOD".to_string()], periods[i]);
        assert_eq!(r.graphs[i].tasks.len(), tasks[i]);
        assert_eq!(r.graphs[i].arcs.len(), arcs[i]);
        assert_eq!(r.graphs[i].deadlines.len(), deadlines[i]);
    }

    let prices = vec![70.1121, 71.4235, 80.491];

    for i in range(0u, 3) {
        assert_eq!(r.tables[i].name, "COMMUN".to_string());
        assert_eq!(r.tables[i].id, i);
        assert_eq!(r.tables[i].attributes["price".to_string()] as f32, prices[i]);
        assert_eq!(r.tables[i].columns.len(), 2);
        assert_eq!(r.tables[i].columns[0].name, "type".to_string());
        assert_eq!(r.tables[i].columns[1].name, "exec_time".to_string());
    }

    let data = vec![
        48.5893, 33.4384, 34.2468, 51.2027, 51.3571, 30.3827, 43.3982, 60.9097,
        36.0322, 34.7446, 45.3479, 31.7221, 49.6842, 52.0635, 44.7690, 37.7183,
        54.7523, 58.4432, 33.1266, 48.2143, 31.2946, 45.9168, 36.4521, 61.6448,
        49.4966, 37.1130, 40.1642, 38.9454, 41.6213, 42.1084, 42.4186, 42.5145,
        34.4180, 33.4178, 32.4243, 63.7925, 50.3810, 51.9030, 46.4714, 35.0566,
        41.8399, 30.1513, 31.7449, 57.3263, 61.2321, 44.9932, 32.0830, 37.9489,
        62.4774, 39.2500,
    ];

    assert_close!(r.tables[1].columns[1].data, data);
}

#[test]
fn parse_032_640() {
    let r = tgff::parse(read_fixture("032_640.tgff").as_slice()).unwrap();

    assert_eq!(r.graphs.len(), 1);
    assert_eq!(r.graphs[0].tasks.len(), 640);
    assert_eq!(r.graphs[0].arcs.len(), 848);
    assert_eq!(r.graphs[0].deadlines.len(), 259);

    assert_eq!(r.tables.len(), 32);
    for table in r.tables.iter() {
        assert_eq!(table.attributes.len(), 1);
        assert_eq!(table.columns.len(), 4);
        for column in table.columns.iter() {
            assert_eq!(column.data.len(), 320);
        }
    }
}

fn read_fixture(name: &'static str) -> String {
    use std::io::fs::PathExtensions;
    let path = Path::new("fixtures").join(name);
    assert!(path.exists());
    std::io::File::open(&path).read_to_string().unwrap()
}
