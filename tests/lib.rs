extern crate assert;

extern crate tgff;

#[test]
fn parse_002_040() {
    let result = tgff::parse(&read_fixture("002_040.tgff")).unwrap();

    assert_eq!(result.attributes["HYPERPERIOD"], 8);
    assert_eq!(result.graphs.len(), 1);
    assert_eq!(result.tables.len(), 2);

    let graph = &result.graphs[0];
    assert_eq!(graph.name, "GRAPH");
    assert_eq!(graph.id, 0);
    assert_eq!(graph.attributes["PERIOD"], 8);
    assert_eq!(graph.tasks.len(), 40);
    assert_eq!(graph.arcs.len(), 52);
    assert_eq!(graph.deadlines.len(), 18);

    let prices = vec![10.5042, 14.8562];
    for i in (0..2) {
        let table = &result.tables[i];
        assert_eq!(table.name, "CORE");
        assert_eq!(table.id, i);
        assert_eq!(table.attributes["price"] as f32, prices[i]);
        assert_eq!(table.columns.len(), 4);
        assert_eq!(table.columns[0].name, "type");
        assert_eq!(table.columns[1].name, "version");
        assert_eq!(table.columns[2].name, "dynamic_power");
        assert_eq!(table.columns[3].name, "execution_time");
    }

    let dynamic_power = vec![
        14.41, 9.38, 14.19, 15.48, 12.15, 16.57, 16.98, 11.02, 17.6, 5.42,
        13.95, 7.08, 5.66, 18.08, 16.51, 5.86, 9.31, 17.25, 8.48, 7.29,
    ];
    assert::close(&result.tables[0].columns[2].data, &dynamic_power, 1e-14);

    let execution_time = vec![
        0.025, 0.019, 0.025, 0.026, 0.022, 0.027, 0.028, 0.021, 0.028, 0.015,
        0.024, 0.017, 0.015, 0.029, 0.027, 0.015, 0.019, 0.028, 0.018, 0.017,
    ];
    assert::close(&result.tables[0].columns[3].data, &execution_time, 1e-14);
}

#[test]
fn parse_032_640() {
    let result = tgff::parse(&read_fixture("032_640.tgff")).unwrap();

    assert_eq!(result.attributes["HYPERPERIOD"], 18);
    assert_eq!(result.graphs.len(), 1);
    assert_eq!(result.tables.len(), 32);

    let graph = &result.graphs[0];
    assert_eq!(graph.tasks.len(), 640);
    assert_eq!(graph.arcs.len(), 848);
    assert_eq!(graph.deadlines.len(), 259);

    for table in result.tables.iter() {
        assert_eq!(table.attributes.len(), 1);
        assert_eq!(table.columns.len(), 4);
        for column in table.columns.iter() {
            assert_eq!(column.data.len(), 320);
        }
    }
}

fn read_fixture(name: &str) -> String {
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    let path = PathBuf::from("tests").join("fixtures").join(name);

    let mut buffer = String::new();
    File::open(&path).unwrap().read_to_string(&mut buffer).unwrap();

    buffer
}
