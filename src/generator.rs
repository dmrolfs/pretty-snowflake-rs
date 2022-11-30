use crate::{AlphabetCodec, Id, IdPrettifier, Label, LabeledRealtimeIdGenerator, MachineNode};
use once_cell::sync::Lazy;

pub type CommonIdGenerator<T> = LabeledRealtimeIdGenerator<T>;

// Since I expect the generator to be set once, at the application start I'm favoring `std::sync::RwLock`
// over tokio::sync::RwLock. This enables the Envelope API to be used in non async circumstances.
static ID_GENERATOR: Lazy<std::sync::RwLock<Option<CommonIdGenerator<()>>>> =
    Lazy::new(|| std::sync::RwLock::new(None));

static DEFAULT_PRETTIFIER: Lazy<IdPrettifier<AlphabetCodec>> = Lazy::new(IdPrettifier::default);

/// Set the ID_GENERATOR to be used by `next_id()`. The `gen` argument may be set up given the
/// assigned `MachineNode` for the process, facilitating uniqueness across nodes in a distributed
/// system. Other configuration, such as prettifier codec, should be consistent across nodes.
pub fn set_id_generator(gen: CommonIdGenerator<()>) {
    let mut generator = ID_GENERATOR.write().unwrap();
    *generator = Some(gen);
}

pub fn prettifier() -> IdPrettifier<AlphabetCodec> {
    let guard = ID_GENERATOR.read().unwrap();
    (*guard)
        .as_ref()
        .map_or_else(|| DEFAULT_PRETTIFIER.clone(), |g| g.prettifier().clone())
}

/// Generate an idea for a labeled type. If a generator was not previously set, a default generator
/// is set with a default `MachineNode` and default `IdPrettifier<AlphabetCodec>`.
pub fn next_id<T: Label>() -> Id<T> {
    let guard = ID_GENERATOR.read().unwrap();

    #[allow(clippy::option_if_let_else)]
    let id = match &*guard {
        Some(g) => g.next_id(),
        None => {
            drop(guard);
            let g = CommonIdGenerator::distributed(MachineNode::default(), prettifier());
            let id = g.next_id();
            set_id_generator(g);
            id
        },
    };

    id.relabel()
}
