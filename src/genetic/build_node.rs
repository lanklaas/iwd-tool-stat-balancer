use std::sync::atomic::{AtomicUsize, Ordering};

use crate::config::stats::Stats;

//Internal
const LAST_NODE_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub age: usize,
    pub stats: Stats,
    pub fitness: Option<()>,
    pub fitness_result: Option<()>,
    pub worker: Option<()>,
    pub promises: (),
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: LAST_NODE_ID.fetch_add(1, Ordering::Relaxed),
            ..Default::default()
        }
    }
}

pub fn build_node() -> Node {
    Node::default()
}
