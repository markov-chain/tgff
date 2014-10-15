use std::collections::HashMap;

/// The content of a TGFF file.
pub struct Content {
    /// Global attributes such as `HYPERPERIOD`.
    pub attributes: HashMap<String, uint>,
    /// Task graphs.
    pub graphs: Vec<Graph>,
    /// Data tables.
    pub tables: Vec<Table>,
}

/// A task graph.
pub struct Graph {
    /// The name of the graph.
    pub name: String,
    /// The ID of the graph.
    pub id: uint,
    /// Graph attributes such as `PERIOD`.
    pub attributes: HashMap<String, uint>,
    /// Vertices representing tasks.
    pub tasks: Vec<Task>,
    /// Edges representing dependencies between the tasks.
    pub arcs: Vec<Arc>,
    /// Hard deadlines of a subset of the tasks.
    pub deadlines: Vec<Deadline>,
}

/// A node in a graph representing a task.
pub struct Task {
    /// The ID of the task.
    pub id: uint,
    /// The type of the task.
    pub kind: uint,
}

/// An edge in a graph connecting two tasks.
pub struct Arc {
    /// The ID of the arc.
    pub id: uint,
    /// The ID of the source task.
    pub from: uint,
    /// The ID of the destination task.
    pub to: uint,
    /// The type of the arc.
    pub kind: uint,
}

/// The deadline of a task.
pub struct Deadline {
    /// The ID of the deadline.
    pub id: uint,
    /// The ID of the task.
    pub on: uint,
    /// The time associated with the deadline.
    pub at: uint,
}

/// A data table.
pub struct Table {
    /// The name of the table.
    pub name: String,
    /// The ID of the table.
    pub id: uint,
    /// Table attributes.
    pub attributes: HashMap<String, f64>,
    /// The columns of the table.
    pub columns: Vec<Column>,
}

/// A column of a table.
pub struct Column {
    /// The name of the column.
    pub name: String,
    /// The data contained in the column.
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
