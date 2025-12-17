use priority_queue::PriorityQueue;
use pyo3::prelude::*;
use std::cmp::Reverse;
use std::collections::HashSet;
use symmetric_matrix::SymmetricMatrix;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct OrderedF64(f64);

impl Eq for OrderedF64 {}

impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

#[derive(Debug, Copy, Clone, Hash)]
#[pyclass]
pub struct Edge {
    u: usize,
    v: usize,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        return (self.u == other.u && self.v == other.v)
            || (self.u == other.v && self.v == other.u);
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl Eq for Edge {}

#[pyclass]
#[derive(Clone)]
pub struct MST {
    pub adj_list: Vec<Vec<usize>>,
    len: usize,
}

#[pymethods]
impl MST {
    #[new]
    fn new(size: usize) -> Self {
        let adj_list = vec![Vec::new(); size];
        return MST { adj_list, len: 0 };
    }

    fn insert_edge(&mut self, edge: &Edge) {
        self.adj_list[edge.u].push(edge.v);
        self.len += 1;
    }
}

pub fn apply_kruscal(graph: &SymmetricMatrix) -> MST {
    let mut sorted_edges = sort_edges(graph);
    let mut mst = MST::new(graph.size);
    let mut included_nodes: HashSet<usize> = HashSet::new();
    included_nodes.insert(0); //Start from node 0

    while mst.len < graph.size - 1 && !sorted_edges.is_empty() {
        let edge = sorted_edges.pop().unwrap().0;

        //Check if adding this edge would create a cycle
        //Only is required to check the arrival node, since we are iterating in order
        if !included_nodes.contains(&edge.v) {
            included_nodes.insert(edge.v);
            mst.insert_edge(&edge);
        }
    }

    return mst;
}

fn sort_edges(matrix: &SymmetricMatrix) -> PriorityQueue<Edge, Reverse<OrderedF64>> {
    let mut queue = PriorityQueue::new(); //Create a new Fibonacci queue
    let mut i = 0;
    let mut j = 1;
    let edge_count = matrix.data.len(); //Number of edges in the graph

    for idx in 0..edge_count
    //Iterate through all edges
    {
        //Create an edge from the matrix and insert it into the queue
        let weight = OrderedF64(matrix.data[idx]);
        let edge = Edge { u: i, v: j };
        queue.push(edge, Reverse(weight));

        //Update indices to point to the next edge in the symmetric matrix
        if j + 1 == matrix.size {
            i += 1;
            j = i + 1;
        } else {
            j += 1;
        }
    }

    return queue;
}
