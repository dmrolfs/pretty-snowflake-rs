mod codec;
mod damm;
mod id;
mod prettifier;

pub use codec::{Alphabet, AlphabetCodec, Codec};
pub use id::Id;
pub use prettifier::IdPrettifier;

use crate::{IdGenerator, SnowflakeIdGenerator};

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

    pub fn distributed(worker_id: i32, datacenter_id: i32, prettifier: IdPrettifier<C>) -> Self {
        Self {
            generator: SnowflakeIdGenerator::<G>::distributed(worker_id, datacenter_id),
            prettifier,
        }
    }

    pub fn next_id(&mut self) -> Id {
        Id::new(self.generator.next_id(), &self.prettifier)
    }
}
