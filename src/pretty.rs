mod codec;
mod damm;
mod id;
mod prettifier;

pub use codec::{Alphabet, AlphabetCodec, Codec};
pub use id::Id;
pub use prettifier::IdPrettifier;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::{IdGenerator, Label, Labeling, MachineNode, SnowflakeIdGenerator};

#[derive(Debug, Clone)]
pub struct PrettyIdGenerator<T: Label, G: IdGenerator, C: Codec> {
    generator: SnowflakeIdGenerator<G>,
    prettifier: IdPrettifier<C>,
    labeling: Rc<Box<dyn Labeling>>,
    marker: PhantomData<T>,
}

impl<T: Label, G: IdGenerator, C: Codec> PrettyIdGenerator<T, G, C> {
    pub fn single_node(prettifier: IdPrettifier<C>) -> Self {
        let labeling = Rc::new(T::labeler());
        let generator = SnowflakeIdGenerator::<G>::single_node();
        Self {
            generator,
            prettifier,
            labeling,
            marker: PhantomData,
        }
    }

    pub fn distributed(machine_node: MachineNode, prettifier: IdPrettifier<C>) -> Self {
        let labeling = Rc::new(T::labeler());
        Self {
            generator: SnowflakeIdGenerator::<G>::distributed(machine_node),
            prettifier,
            labeling,
            marker: PhantomData,
        }
    }

    pub fn next_id(&mut self) -> Id<T> {
        Id::new(self.labeling.label(), self.generator.next_id(), &self.prettifier)
    }
}
