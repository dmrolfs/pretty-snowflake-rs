use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use crate::DatacenterWorker;
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
    datacenter_worker: DatacenterWorker,
    worker: Worker,
    marker: PhantomData<G>,
}

impl<G> Default for SnowflakeIdGenerator<G> {
    fn default() -> Self {
        let datacenter_worker = DatacenterWorker::default();
        let worker = Worker::new(datacenter_worker.datacenter_id, datacenter_worker.worker_id);
        Self { datacenter_worker, worker, marker: PhantomData }
    }
}

impl<G: IdGenerator> SnowflakeIdGenerator<G> {
    pub fn single_node() -> Self {
        Self::distributed(DatacenterWorker::default())
    }

    pub fn distributed(datacenter_worker: DatacenterWorker) -> Self {
        let worker = Worker::new(datacenter_worker.datacenter_id, datacenter_worker.worker_id);
        Self { datacenter_worker, worker, marker: PhantomData }
    }

    pub fn next_id(&mut self) -> i64 {
        G::next_id(&mut self.worker)
    }
}

impl<G> PartialEq for SnowflakeIdGenerator<G> {
    fn eq(&self, other: &Self) -> bool {
        self.datacenter_worker == other.datacenter_worker
    }
}

impl<G> Eq for SnowflakeIdGenerator<G> {}

impl<G> Ord for SnowflakeIdGenerator<G> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.datacenter_worker.cmp(&other.datacenter_worker)
    }
}

impl<G> PartialOrd for SnowflakeIdGenerator<G> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<G> Hash for SnowflakeIdGenerator<G> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.datacenter_worker.hash(state);
    }
}
