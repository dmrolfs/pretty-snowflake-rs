pub use codec::{Alphabet, AlphabetCodec, Codec};
pub use id::Id;
pub use prettifier::IdPrettifier;
use std::marker::PhantomData;

#[cfg(test)]
mod tests;

mod codec;
mod damm;
mod id;
mod prettifier;

use crate::{
    Generator, IdGenerator, Label, Labeling, LazyGenerator, MachineNode, RealTimeGenerator, SnowflakeIdGenerator,
};

#[derive(Debug, Clone)]
pub struct PrettyIdGenerator<T, L, G, C>
where
    L: Labeling + Clone,
    G: IdGenerator,
    C: Codec,
{
    generator: SnowflakeIdGenerator<G>,
    prettifier: IdPrettifier<C>,
    labeling: L,
    marker: PhantomData<T>,
}

impl<T: Label> Default for PrettyIdGenerator<T, <T as Label>::Labeler, RealTimeGenerator, AlphabetCodec> {
    fn default() -> Self {
        Self::single_node(IdPrettifier::default())
    }
}

impl<T: Label> PrettyIdGenerator<T, <T as Label>::Labeler, RealTimeGenerator, AlphabetCodec> {
    pub fn distributed_realtime(machine_node: MachineNode) -> Self {
        Self::distributed(machine_node, IdPrettifier::default())
    }
}

impl<T: Label> Default for PrettyIdGenerator<T, <T as Label>::Labeler, Generator, AlphabetCodec> {
    fn default() -> Self {
        Self::single_node(IdPrettifier::default())
    }
}

impl<T: Label> PrettyIdGenerator<T, <T as Label>::Labeler, Generator, AlphabetCodec> {
    pub fn distributed_basic(machine_node: MachineNode) -> Self {
        Self::distributed(machine_node, IdPrettifier::default())
    }
}

impl<T: Label> Default for PrettyIdGenerator<T, <T as Label>::Labeler, LazyGenerator, AlphabetCodec> {
    fn default() -> Self {
        Self::single_node(IdPrettifier::default())
    }
}

impl<T: Label> PrettyIdGenerator<T, <T as Label>::Labeler, LazyGenerator, AlphabetCodec> {
    pub fn distributed_lazy(machine_node: MachineNode) -> Self {
        Self::distributed(machine_node, IdPrettifier::default())
    }
}

impl<T, G, C> PrettyIdGenerator<T, <T as Label>::Labeler, G, C>
where
    T: Label,
    G: IdGenerator,
    C: Codec,
{
    pub fn single_node(prettifier: IdPrettifier<C>) -> Self {
        let labeling = T::labeler();
        let generator = SnowflakeIdGenerator::single_node();
        Self {
            generator,
            prettifier,
            labeling,
            marker: PhantomData,
        }
    }

    pub fn distributed(machine_node: MachineNode, prettifier: IdPrettifier<C>) -> Self {
        let labeling = T::labeler();
        let generator = SnowflakeIdGenerator::distributed(machine_node);
        Self {
            generator,
            prettifier,
            labeling,
            marker: PhantomData,
        }
    }

    pub const fn prettifier(&self) -> &IdPrettifier<C> {
        &self.prettifier
    }
}

impl<T, L, G, C> PrettyIdGenerator<T, L, G, C>
where
    L: Labeling + Clone,
    G: IdGenerator,
    C: Codec,
{
    pub fn single_node_labeling(labeling: L, prettifier: IdPrettifier<C>) -> Self {
        let generator = SnowflakeIdGenerator::single_node();
        Self {
            generator,
            prettifier,
            labeling,
            marker: PhantomData,
        }
    }

    pub fn distributed_labeling(machine_node: MachineNode, labeling: L, prettifier: IdPrettifier<C>) -> Self {
        let generator = SnowflakeIdGenerator::distributed(machine_node);
        Self {
            generator,
            prettifier,
            labeling,
            marker: PhantomData,
        }
    }

    pub fn next_id(&self) -> Id<T> {
        Id::new(self.labeling.label(), self.generator.next_id(), &self.prettifier)
    }
}

// #[cfg(test)]
// mod tests {
//     use pretty_assertions::assert_eq;
//
//     use super::*;
//     use crate::{CustomLabeling, LazyGenerator};
//
//     struct NonLabelZed;
//
//     struct Foo;
//
//     impl Label for Foo {
//         type Labeler = CustomLabeling;
//
//         fn labeler() -> Self::Labeler {
//             CustomLabeling::new("MyFooferNut")
//         }
//     }
//
//     #[test]
//     fn test_non_label_custom_generator() {
//         let mut gen: PrettyIdGenerator<NonLabelZed, CustomLabeling, LazyGenerator, AlphabetCodec>
// =             PrettyIdGenerator::single_node_labeling(CustomLabeling::new("Zedster"),
// IdPrettifier::default());
//
//         let actual = gen.next_id();
//         assert_eq!(format!("{:?}", actual), format!("Zedster::{}", actual))
//     }
//
//     #[test]
//     fn test_labeled_generator() {
//         let mut gen: PrettyIdGenerator<Foo, CustomLabeling, LazyGenerator, AlphabetCodec> =
//             PrettyIdGenerator::single_node(IdPrettifier::default());
//
//         let actual = gen.next_id();
//         assert_eq!(format!("{:?}", actual), format!("MyFooferNut::{}", actual))
//     }
// }
