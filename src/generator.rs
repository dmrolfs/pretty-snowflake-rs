use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use crate::MachineNode;
use snowflake::SnowflakeIdGenerator as Worker;

pub trait IdGenerator {
    fn next_id(worker: &mut Worker) -> i64;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct RealTimeGenerator;

#[derive(Debug, Default, Copy, Clone)]
pub struct Generator;

#[derive(Debug, Default, Copy, Clone)]
pub struct LazyGenerator;

impl IdGenerator for RealTimeGenerator {
    fn next_id(worker: &mut Worker) -> i64 {
        worker.real_time_generate()
    }
}

impl IdGenerator for Generator {
    fn next_id(worker: &mut Worker) -> i64 {
        worker.generate()
    }
}

impl IdGenerator for LazyGenerator {
    fn next_id(worker: &mut Worker) -> i64 {
        worker.lazy_generate()
    }
}

/// Generates time-based unique ids. Each node should have a different workerId.
#[derive(Debug, Clone)]
pub struct SnowflakeIdGenerator<G> {
    machine_node: MachineNode,
    worker: Worker,
    marker: PhantomData<G>,
}

impl<G> Default for SnowflakeIdGenerator<G> {
    fn default() -> Self {
        let machine_node = MachineNode::default();
        let worker = Worker::new(machine_node.machine_id, machine_node.node_id);
        Self { machine_node, worker, marker: PhantomData }
    }
}

impl<G: IdGenerator> SnowflakeIdGenerator<G> {
    pub fn single_node() -> Self {
        Self::distributed(MachineNode::default())
    }

    pub fn distributed(machine_node: MachineNode) -> Self {
        let worker = Worker::new(machine_node.machine_id, machine_node.node_id);
        Self { machine_node, worker, marker: PhantomData }
    }

    pub fn next_id(&mut self) -> i64 {
        G::next_id(&mut self.worker)
    }
}

impl<G> PartialEq for SnowflakeIdGenerator<G> {
    fn eq(&self, other: &Self) -> bool {
        self.machine_node == other.machine_node
    }
}

impl<G> Eq for SnowflakeIdGenerator<G> {}

impl<G> Ord for SnowflakeIdGenerator<G> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.machine_node.cmp(&other.machine_node)
    }
}

impl<G> PartialOrd for SnowflakeIdGenerator<G> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<G> Hash for SnowflakeIdGenerator<G> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.machine_node.hash(state);
    }
}
