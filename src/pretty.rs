mod codec;
mod damm;
mod id;
mod prettifier;

pub use codec::{Alphabet, AlphabetCodec, Codec};
pub use id::Id;
pub use prettifier::IdPrettifier;

use crate::{IdGenerator, Labeling, MachineNode, SnowflakeIdGenerator};

#[derive(Debug, Clone)]
pub struct PrettyIdGenerator<G: IdGenerator, L: Labeling, C: Codec> {
    generator: SnowflakeIdGenerator<G>,
    prettifier: IdPrettifier<C>,
    labeling: L,
}

impl<G, L, C> Default for PrettyIdGenerator<G, L, C>
where
    G: IdGenerator + Default,
    L: Labeling + Default,
    C: Codec + Default,
{
    fn default() -> Self {
        Self {
            generator: SnowflakeIdGenerator::<G>::default(),
            prettifier: IdPrettifier::<C>::default(),
            labeling: L::default(),
        }
    }
}

impl<G: IdGenerator, L: Labeling, C: Codec> PrettyIdGenerator<G, L, C> {
    pub fn single_node(labeling: L, prettifier: IdPrettifier<C>) -> Self {
        let generator = SnowflakeIdGenerator::<G>::single_node();
        Self { generator, labeling, prettifier }
    }

    pub fn distributed(labeling: L, machine_node: MachineNode, prettifier: IdPrettifier<C>) -> Self {
        Self {
            generator: SnowflakeIdGenerator::<G>::distributed(machine_node),
            labeling,
            prettifier,
        }
    }

    pub fn next_id(&mut self) -> Id {
        Id::new(self.labeling.label(), self.generator.next_id(), &self.prettifier)
    }
}
