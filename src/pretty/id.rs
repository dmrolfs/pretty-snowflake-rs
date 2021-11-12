use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use crate::Label;
use serde::{Deserialize, Serialize};

use crate::pretty::codec::Codec;
use crate::pretty::prettifier::IdPrettifier;

#[derive(Serialize, Deserialize)]
pub struct Id<T: Label + ?Sized> {
    pub label: String,
    snowflake: i64,
    pretty: String, // todo: convert into [char; N] form to support Cpy semantics
    marker: PhantomData<T>,
}

impl<T: Label + ?Sized> Id<T> {
    pub fn new<C: Codec>(label: impl Into<String>, snowflake: i64, prettifier: &IdPrettifier<C>) -> Self {
        Self {
            label: label.into(),
            snowflake,
            pretty: prettifier.prettify(snowflake),
            marker: PhantomData,
        }
    }

    pub fn direct(label: impl Into<String>, snowflake: i64, pretty: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            snowflake,
            pretty: pretty.into(),
            marker: PhantomData,
        }
    }

    pub fn reapply<B: Label + ?Sized>(&self) -> Id<B> {
        let b_labeler = B::labeler();
        Id {
            label: b_labeler.label().into_owned(),
            snowflake: self.snowflake,
            pretty: self.pretty.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: Label + ?Sized> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            label: self.label.clone(),
            snowflake: self.snowflake,
            pretty: self.pretty.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: Label + ?Sized> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.debug_struct("Id")
                .field("label", &self.label)
                .field("snowflake", &self.snowflake)
                .field("pretty", &self.pretty)
                .finish()
        } else if self.label.is_empty() {
            f.write_str(self.pretty.as_str())
        } else {
            f.write_fmt(format_args!("{}::{}", self.label, self.pretty))
        }
    }
}

impl<T: Label + ?Sized> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.snowflake)
        } else {
            write!(f, "{}", self.pretty)
        }
    }
}

impl<T: Label + ?Sized> Into<i64> for Id<T> {
    fn into(self) -> i64 {
        self.snowflake
    }
}

impl<T: Label + ?Sized> Into<String> for Id<T> {
    fn into(self) -> String {
        self.pretty
    }
}

impl<T: Label + ?Sized> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.snowflake == other.snowflake
    }
}

impl<T: Label + ?Sized> Eq for Id<T> {}

impl<T: Label + ?Sized> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.snowflake.cmp(&other.snowflake)
    }
}

impl<T: Label + ?Sized> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Label + ?Sized> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.snowflake.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::{AlphabetCodec, Id, IdPrettifier, Label, Labeling, MakeLabeling, PrettyIdGenerator, RealTimeGenerator};
    use pretty_assertions::assert_eq;

    struct Foo;
    impl Label for Foo {
        fn labeler() -> Box<dyn Labeling> {
            Box::new(MakeLabeling::<Foo>::default())
        }
    }
    fn make_generator<T: Label>() -> PrettyIdGenerator<T, RealTimeGenerator, AlphabetCodec> {
        PrettyIdGenerator::single_node(IdPrettifier::<AlphabetCodec>::default())
    }

    #[test]
    fn test_partial_ord() {
        let mut generator = make_generator::<()>();
        let a = generator.next_id();
        let b = generator.next_id();
        assert!(a < b);
    }

    #[test]
    fn test_display() {
        let mut generator = make_generator();
        let a: Id<Foo> = generator.next_id();
        assert_eq!(format!("{}", a), a.pretty);
    }

    #[test]
    fn test_alternate_display() {
        let mut generator = make_generator();
        let a: Id<i64> = generator.next_id();
        assert_eq!(format!("{:#}", a), a.snowflake.to_string());
    }

    #[test]
    fn test_debug() {
        let mut generator = make_generator();
        let a: Id<Foo> = generator.next_id();
        assert_eq!(format!("{:?}", a), format!("Foo::{}", a.pretty));
    }

    #[test]
    fn test_alternate_debug() {
        let mut generator = make_generator();
        let a: Id<Foo> = generator.next_id();
        assert_eq!(
            format!("{:#?}", a),
            format!(
                "Id {{\n    label: \"{}\",\n    snowflake: {},\n    pretty: \"{}\",\n}}",
                a.label, a.snowflake, a.pretty
            )
        );
    }

    #[test]
    fn test_id_cross_conversion() {
        let mut generator = make_generator();
        let a: Id<String> = generator.next_id();
        let before = format!("{:?}", a);
        assert_eq!(format!("String::{}", a.pretty), before);

        let b: Id<usize> = a.reapply();
        let after = format!("{:?}", b);
        assert_eq!(format!("usize::{}", b.pretty), after);
    }
}
