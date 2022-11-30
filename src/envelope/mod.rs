use crate::Id;
use iso8601_timestamp::Timestamp;

#[allow(clippy::module_inception)]
mod envelope;

mod metadata;

mod serde_impl;

pub use envelope::{Envelope, IntoEnvelope};
pub use metadata::MetaData;
pub use serde_impl::*;

/// Type has correlation identifier.
pub trait Correlation {
    type Correlated: Sized + Sync;
    fn correlation(&self) -> &Id<Self::Correlated>;
}

/// Type has received at timestamp.
pub trait ReceivedAt {
    fn recv_timestamp(&self) -> Timestamp;
}
