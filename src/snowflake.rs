use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use snowflake::SnowflakeIdGenerator as Worker;

use crate::MachineNode;

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Id(i64);

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "Id({})", self.0)
        } else {
            f.debug_tuple("Id").field(&self.0).finish()
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Id> for i64 {
    fn from(id: Id) -> Self {
        id.0
    }
}

impl From<i64> for Id {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl FromStr for Id {
    type Err = std::num::ParseIntError;

    fn from_str(rep: &str) -> Result<Self, Self::Err> {
        Ok(i64::from_str(rep)?.into())
    }
}

pub trait IdGenerator {
    fn next_id(worker: &mut Worker) -> Id;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct RealTimeGenerator;

#[derive(Debug, Default, Copy, Clone)]
pub struct Generator;

#[derive(Debug, Default, Copy, Clone)]
pub struct LazyGenerator;

impl IdGenerator for RealTimeGenerator {
    fn next_id(worker: &mut Worker) -> Id {
        worker.real_time_generate().into()
    }
}

impl IdGenerator for Generator {
    fn next_id(worker: &mut Worker) -> Id {
        worker.generate().into()
    }
}

impl IdGenerator for LazyGenerator {
    fn next_id(worker: &mut Worker) -> Id {
        worker.lazy_generate().into()
    }
}

/// Generates time-based unique ids. Each node should have a different workerId.
#[derive(Debug, Clone)]
pub struct SnowflakeIdGenerator<G> {
    machine_node: MachineNode,
    worker: RefCell<Worker>,
    marker: PhantomData<G>,
}

impl<G> Default for SnowflakeIdGenerator<G> {
    fn default() -> Self {
        let machine_node = MachineNode::default();
        let worker = Worker::new(machine_node.machine_id, machine_node.node_id);
        Self {
            machine_node,
            worker: RefCell::new(worker),
            marker: PhantomData,
        }
    }
}

impl<G: IdGenerator> SnowflakeIdGenerator<G> {
    pub fn single_node() -> Self {
        Self::distributed(MachineNode::default())
    }

    pub fn distributed(machine_node: MachineNode) -> Self {
        let worker = Worker::new(machine_node.machine_id, machine_node.node_id);
        Self {
            machine_node,
            worker: RefCell::new(worker),
            marker: PhantomData,
        }
    }

    pub fn next_id(&self) -> Id {
        let mut w = self.worker.borrow_mut();
        G::next_id(&mut w)
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
