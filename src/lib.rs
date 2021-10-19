mod generator;
mod pretty;

pub use generator::{Generator, IdGenerator, LazyGenerator, RealTimeGenerator, SnowflakeIdGenerator};
pub use pretty::{Alphabet, AlphabetCodec, Codec, Id, IdPrettifier, PrettyIdGenerator};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Copy, Clone, Validate, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DatacenterWorker {
    #[validate(range(min = 1))]
    pub worker_id: i32,

    #[validate(range(min = 1))]
    pub datacenter_id: i32,
}

impl fmt::Display for DatacenterWorker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}::{})", self.datacenter_id, self.worker_id)
    }
}

impl Default for DatacenterWorker {
    fn default() -> Self {
        Self { worker_id: 1, datacenter_id: 1 }
    }
}

impl DatacenterWorker {
    pub fn new(worker_id: i32, datacenter_id: i32) -> Result<Self, ValidationErrors> {
        let result = Self { worker_id, datacenter_id };
        result.validate()?;
        Ok(result)
    }
}

impl Ord for DatacenterWorker {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.datacenter_id.cmp(&other.datacenter_id) {
            Ordering::Equal => self.worker_id.cmp(&other.worker_id),
            o => o,
        }
    }
}

impl PartialOrd for DatacenterWorker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
