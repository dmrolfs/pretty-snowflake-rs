use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::pretty::codec::Codec;
use crate::pretty::prettifier::IdPrettifier;

#[derive(Clone, Serialize, Deserialize)]
pub struct Id {
    pub label: String,
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

impl fmt::Debug for Id {
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

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.snowflake)
        } else {
            write!(f, "{}", self.pretty)
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
    use pretty_assertions::assert_eq;
    use trim_margin::MarginTrimmable;

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
        assert_eq!(format!("{}", a), a.pretty);
    }

    #[test]
    fn test_alternate_display() {
        let mut generator = make_generator();
        let a = generator.next_id();
        assert_eq!(format!("{:#}", a), a.snowflake.to_string());
    }

    #[test]
    fn test_debug() {
        let mut generator = make_generator();
        let a = generator.next_id();
        assert_eq!(format!("{:?}", a), format!("Foo::{}", a.pretty));
    }

    #[test]
    fn test_alternate_debug() {
        let mut generator = make_generator();
        let a = generator.next_id();
        let debug_template = assert_eq!(
            format!("{:#?}", a),
            format!(
                "Id {{\n    label: \"{}\",\n    snowflake: {},\n    pretty: \"{}\",\n}}",
                a.label, a.snowflake, a.pretty
            )
        );
    }
}
