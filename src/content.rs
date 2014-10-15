use std::collections::HashMap;

/// The content of a TGFF file.
pub struct Content {
    pub attributes: HashMap<String, uint>,
    pub graphs: Vec<Graph>,
    pub tables: Vec<Table>,
}

/// A graph.
pub struct Graph {
    pub name: String,
    pub id: uint,
    pub attributes: HashMap<String, uint>,
    pub tasks: Vec<Task>,
    pub arcs: Vec<Arc>,
    pub deadlines: Vec<Deadline>,
}

/// A TASK entry of a graph.
pub struct Task {
    pub id: uint,
    pub kind: uint,
}

/// An ARC entry of a graph.
pub struct Arc {
    pub id: uint,
    pub from: uint,
    pub to: uint,
    pub kind: uint,
}

/// A HARD_DEADLINE entry of a graph.
pub struct Deadline {
    pub id: uint,
    pub on: uint,
    pub at: uint,
}

/// A table.
pub struct Table {
    pub name: String,
    pub id: uint,
    pub attributes: HashMap<String, f64>,
    pub columns: Vec<Column>,
}

/// A column of a table.
pub struct Column {
    pub name: String,
    pub data: Vec<f64>,
}

#[inline]
pub fn new() -> Content {
    Content {
        attributes: HashMap::new(),
        graphs: Vec::new(),
        tables: Vec::new(),
    }
}

#[inline]
pub fn new_graph(name: String, id: uint) -> Graph {
    Graph {
        name: name,
        id: id,
        attributes: HashMap::new(),
        tasks: Vec::new(),
        arcs: Vec::new(),
        deadlines: Vec::new(),
    }
}

#[inline]
pub fn new_task(id: uint, kind: uint) -> Task {
    Task { id: id, kind: kind }
}

#[inline]
pub fn new_arc(id: uint, from: uint, to: uint, kind: uint) -> Arc {
    Arc { id: id, from: from, to: to, kind: kind }
}

#[inline]
pub fn new_deadline(id: uint, on: uint, at: uint) -> Deadline {
    Deadline { id: id, on: on, at: at }
}

#[inline]
pub fn new_table(name: String, id: uint) -> Table {
    Table {
        name: name,
        id: id,
        attributes: HashMap::new(),
        columns: Vec::new(),
    }
}

#[inline]
pub fn new_column(name: String) -> Column {
    Column { name: name, data: vec![] }
}
