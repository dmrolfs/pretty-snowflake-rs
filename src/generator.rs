use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use snowflake::SnowflakeIdGenerator as Worker;

pub trait IdGenerator {
    fn next_id(worker: &mut Worker) -> i64;
}

#[derive(Debug, Default)]
pub struct RealTimeGenerator;

#[derive(Debug, Default)]
pub struct Generator;

#[derive(Debug, Default)]
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
    worker_id: i32,
    datacenter_id: i32,
    worker: Worker,
    marker: PhantomData<G>,
}

impl<G> Default for SnowflakeIdGenerator<G> {
    fn default() -> Self {
        let worker_id = 1;
        let datacenter_id = 1;
        let worker = Worker::new(datacenter_id, worker_id);
        Self {
            worker_id,
            datacenter_id,
            worker,
            marker: PhantomData,
        }
    }
}

impl<G: IdGenerator> SnowflakeIdGenerator<G> {
    pub fn single_node() -> Self {
        Self::distributed(1, 1)
    }

    pub fn distributed(worker_id: i32, datacenter_id: i32) -> Self {
        let worker = Worker::new(datacenter_id, worker_id);
        Self {
            worker_id,
            datacenter_id,
            worker,
            marker: PhantomData,
        }
    }

    pub fn next_id(&mut self) -> i64 {
        G::next_id(&mut self.worker)
    }
}

impl<G> PartialEq for SnowflakeIdGenerator<G> {
    fn eq(&self, other: &Self) -> bool {
        self.datacenter_id == other.datacenter_id && self.worker_id == other.worker_id
    }
}

impl<G> Eq for SnowflakeIdGenerator<G> {}

impl<G> Ord for SnowflakeIdGenerator<G> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.datacenter_id.cmp(&other.datacenter_id) {
            Ordering::Equal => self.worker_id.cmp(&other.worker_id),
            o => o,
        }
    }
}

impl<G> PartialOrd for SnowflakeIdGenerator<G> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<G> Hash for SnowflakeIdGenerator<G> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.datacenter_id.hash(state);
        self.worker_id.hash(state);
    }
}
