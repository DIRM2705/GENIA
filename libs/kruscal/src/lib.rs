
use priority_queue::PriorityQueue;
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

pub struct MST
{
    adj_list : Vec<Vec<usize>>,
    len : usize
}

impl MST
{
    pub fn new(size : usize) -> Self
    {
        let adj_list = vec![Vec::new(); size];
        return MST
        {
            adj_list,
            len : 0
        }
    }

    pub fn insert_edge(&mut self, edge : &Edge)
    {
        self.adj_list[edge.u].push(edge.v);
        self.adj_list[edge.v].push(edge.u);
        self.len += 1;
    }
}

pub fn apply_kruscal(graph: &SymmetricMatrix) -> MST {
    let mut sorted_edges = sort_edges(graph);
    let mut mst_nodes: HashSet<usize> = HashSet::new();
    let mut mst = MST::new(graph.size);

    while mst.len < graph.size - 1 && !sorted_edges.is_empty() {
        let edge_op = sorted_edges.pop();
        println!("Edge {:?} popped from queue", edge_op);
        let edge = edge_op.unwrap().0;

        if !mst_nodes.contains(&edge.u) || !mst_nodes.contains(&edge.v) {
            mst.insert_edge(&edge);
            mst_nodes.insert(edge.u);
            mst_nodes.insert(edge.v);
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