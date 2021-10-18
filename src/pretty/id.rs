use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::pretty::codec::Codec;
use crate::pretty::prettifier::IdPrettifier;

#[derive(Debug, Clone)]
pub struct Id {
    snowflake: i64,
    pretty: String,
}

impl Id {
    pub fn new<C: Codec>(snowflake: i64, prettifier: &IdPrettifier<C>) -> Self {
        Self { snowflake, pretty: prettifier.prettify(snowflake) }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty)
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
