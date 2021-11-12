use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::pretty::codec::Codec;
use crate::pretty::prettifier::IdPrettifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Id {
    label: String,
    snowflake: i64,
    pretty: String, // todo: convert into [char; N] form to support Cpy semantics
}

impl Id {
    pub fn new<C: Codec>(label: impl Into<String>, snowflake: i64, prettifier: &IdPrettifier<C>) -> Self {
        Self {
            label: label.into(),
            snowflake,
            pretty: prettifier.prettify(snowflake),
        }
    }

    pub fn direct(label: impl Into<String>, snowflake: i64, pretty: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            snowflake,
            pretty: pretty.into(),
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.snowflake)
        } else {
            if self.label.is_empty() {
                write!(f, "{}", self.pretty)
            } else {
                write!(f, "{}::{}", self.label, self.pretty)
            }
        }
    }
}

impl Into<i64> for Id {
    fn into(self) -> i64 {
        self.snowflake
    }
}

impl Into<String> for Id {
    fn into(self) -> String {
        self.pretty
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.snowflake == other.snowflake
    }
}

impl Eq for Id {}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> Ordering {
        self.snowflake.cmp(&other.snowflake)
    }
}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Id {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.snowflake.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::labeling::CustomLabeling;
    use crate::{AlphabetCodec, IdPrettifier, PrettyIdGenerator, RealTimeGenerator};

    fn make_generator() -> PrettyIdGenerator<RealTimeGenerator, CustomLabeling, AlphabetCodec> {
        PrettyIdGenerator::single_node(CustomLabeling::new("Foo"), IdPrettifier::<AlphabetCodec>::default())
    }

    #[test]
    fn test_partial_ord() {
        let mut generator = make_generator();
        let a = generator.next_id();
        let b = generator.next_id();
        assert!(a < b);
    }

    #[test]
    fn test_display() {
        let mut generator = make_generator();
        let a = generator.next_id();
        assert_eq!(format!("{}", a), format!("Foo::{}", a.pretty));
    }

    #[test]
    fn test_alternate_display() {
        let mut generator = make_generator();
        let a = generator.next_id();
        assert_eq!(format!("{:#}", a), a.snowflake.to_string());
    }
}
