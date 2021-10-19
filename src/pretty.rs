mod codec;
mod damm;
mod id;
mod prettifier;

pub use codec::{Alphabet, AlphabetCodec, Codec};
pub use id::Id;
pub use prettifier::IdPrettifier;

use crate::{DatacenterWorker, IdGenerator, SnowflakeIdGenerator};

#[derive(Debug, Clone)]
pub struct PrettyIdGenerator<G: IdGenerator, C: Codec> {
    generator: SnowflakeIdGenerator<G>,
    prettifier: IdPrettifier<C>,
}

impl<G, C> Default for PrettyIdGenerator<G, C>
where
    G: IdGenerator + Default,
    C: Codec + Default,
{
    fn default() -> Self {
        Self {
            generator: SnowflakeIdGenerator::<G>::default(),
            prettifier: IdPrettifier::<C>::default(),
        }
    }
}

impl<G: IdGenerator, C: Codec> PrettyIdGenerator<G, C> {
    pub fn single_node(prettifier: IdPrettifier<C>) -> Self {
        let generator = SnowflakeIdGenerator::<G>::single_node();
        Self { generator, prettifier }
    }

    pub fn distributed(datacenter_worker: DatacenterWorker, prettifier: IdPrettifier<C>) -> Self {
        Self {
            generator: SnowflakeIdGenerator::<G>::distributed(datacenter_worker),
            prettifier,
        }
    }

    pub fn next_id(&mut self) -> Id {
        Id::new(self.generator.next_id(), &self.prettifier)
    }
}
