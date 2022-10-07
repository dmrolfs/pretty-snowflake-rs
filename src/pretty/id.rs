use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use pretty_type_name::pretty_type_name;
use serde::de::{self, Deserialize, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserializer;
use smol_str::SmolStr;

use crate::pretty::codec::Codec;
use crate::pretty::prettifier::IdPrettifier;
use crate::snowflake::Id as SnowflakeId;
use crate::{Label, Labeling};

const ID_SNOWFLAKE: &str = "snowflake";
const ID_PRETTY: &str = "pretty";
const FIELDS: [&str; 2] = [ID_SNOWFLAKE, ID_PRETTY];

pub struct Id<T> {
    label: SmolStr,
    snowflake: SnowflakeId,
    pretty: SmolStr, // todo: convert into [char; N] form to support Cpy semantics
    marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new<C: Codec>(
        label: impl AsRef<str>, snowflake: impl Into<SnowflakeId>, prettifier: &IdPrettifier<C>,
    ) -> Self {
        let snowflake: SnowflakeId = snowflake.into();
        Self {
            label: SmolStr::new(label.as_ref()),
            snowflake,
            pretty: SmolStr::new_inline(&prettifier.prettify(snowflake)),
            marker: PhantomData,
        }
    }

    pub fn direct(label: impl AsRef<str>, snowflake: impl Into<SnowflakeId>, pretty: impl AsRef<str>) -> Self {
        Self {
            label: SmolStr::new(label.as_ref()),
            snowflake: snowflake.into(),
            pretty: SmolStr::new_inline(pretty.as_ref()),
            marker: PhantomData,
        }
    }

    pub fn relabel<B: Label>(&self) -> Id<B> {
        let b_labeler = B::labeler();
        Id {
            label: SmolStr::new(b_labeler.label().as_ref()),
            snowflake: self.snowflake,
            pretty: self.pretty.clone(),
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    #[inline]
    pub fn pretty(&self) -> &str {
        self.pretty.as_str()
    }

    #[inline]
    pub fn num(&self) -> i64 {
        self.snowflake.into()
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            label: self.label.clone(),
            snowflake: self.snowflake,
            pretty: self.pretty.clone(),
            marker: PhantomData,
        }
    }
}

impl<T> fmt::Debug for Id<T> {
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

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.snowflake)
        } else {
            write!(f, "{}", self.pretty)
        }
    }
}

impl<T> From<Id<T>> for SnowflakeId {
    fn from(id: Id<T>) -> Self {
        id.snowflake
    }
}

impl<T> From<Id<T>> for i64 {
    fn from(id: Id<T>) -> Self {
        id.snowflake.into()
    }
}

impl<T> From<Id<T>> for String {
    fn from(id: Id<T>) -> Self {
        id.pretty.to_string()
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.snowflake == other.snowflake
    }
}

impl<T> Eq for Id<T> {}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.snowflake.cmp(&other.snowflake)
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.snowflake.hash(state);
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Id", 2)?;
        state.serialize_field(ID_SNOWFLAKE, &self.snowflake)?;
        state.serialize_field(ID_PRETTY, &self.pretty)?;
        state.end()
    }
}

impl<'de, T: Label> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Snowflake,
            Pretty,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.write_str("`snowflake` or `pretty`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            ID_SNOWFLAKE => Ok(Self::Value::Snowflake),
                            ID_PRETTY => Ok(Self::Value::Pretty),
                            _ => Err(de::Error::unknown_field(value, &FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct IdVisitor<T> {
            marker: PhantomData<T>,
        }

        impl<T> IdVisitor<T> {
            pub const fn new() -> Self {
                Self { marker: PhantomData }
            }
        }

        impl<'de, T: Label> Visitor<'de> for IdVisitor<T> {
            type Value = Id<T>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(format!("struct Id<{}>", pretty_type_name::<T>()).as_str())
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let snowflake: i64 = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let pretty: String = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let labeler = <T as Label>::labeler();
                let label = labeler.label();
                Ok(Id::direct(label, snowflake, pretty))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut snowflake = None;
                let mut pretty = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Snowflake => {
                            if snowflake.is_some() {
                                return Err(de::Error::duplicate_field(ID_SNOWFLAKE));
                            }
                            snowflake = Some(map.next_value()?);
                        },
                        Field::Pretty => {
                            if pretty.is_some() {
                                return Err(de::Error::duplicate_field(ID_PRETTY));
                            }
                            pretty = Some(map.next_value()?);
                        },
                    }
                }

                let snowflake: i64 = snowflake.ok_or_else(|| de::Error::missing_field(ID_SNOWFLAKE))?;
                let pretty: String = pretty.ok_or_else(|| de::Error::missing_field(ID_PRETTY))?;
                let labeler = <T as Label>::labeler();
                let label = labeler.label();
                Ok(Id::direct(label, snowflake, pretty))
            }
        }

        deserializer.deserialize_struct("Id", &FIELDS, IdVisitor::<T>::new())
    }
}
