use std::collections::HashMap;

/// The content of a TGFF file.
pub struct Content {
    attributes: HashMap<String, String>,
    graphs: Vec<Graph>,
    tables: Vec<Table>,
}

/// A graph.
pub struct Graph {
    name: String,
    id: uint,
    attributes: HashMap<String, String>,
    tasks: Vec<Task>,
    arcs: Vec<Arc>,
    deadlines: Vec<Deadline>,
}

/// A TASK entry of a graph.
pub struct Task {
    name: String,
    kind: uint,
}

/// An ARC entry of a graph.
pub struct Arc {
    name: String,
    from: uint,
    to: uint,
    kind: uint,
}

/// A HARD_DEADLINE entry of a graph.
pub struct Deadline {
    name: String,
    on: uint,
    at: uint,
}

/// A table.
pub struct Table {
    name: String,
    id: uint,
    attributes: HashMap<String, String>,
    columns: Vec<Column>,
}

/// A column of a table.
pub struct Column {
    name: String,
    data: Vec<f64>,
}

impl Content {
    pub fn new() -> Content {
        Content {
            attributes: HashMap::new(),
            graphs: Vec::new(),
            tables: Vec::new(),
        }
    }

    pub fn set_attribute(&mut self, name: String, value: String) {
        self.attributes.insert(name, value);
    }
}
