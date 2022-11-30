use crate::envelope::Envelope;
use crate::Label;
use serde::ser::SerializeStruct;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;

impl<T> Serialize for Envelope<T>
where
    T: Serialize + Label + Send,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Envelope", 2)?;
        s.serialize_field("metadata", self.metadata())?;
        s.serialize_field("content", self.deref())?;
        s.end()
    }
}

impl<'de, T> Deserialize<'de> for Envelope<T>
where
    T: de::DeserializeOwned + Label + Send,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            MetaData,
            Content,
        }

        impl Field {
            const META_DATA: &'static str = "metadata";
            const CONTENT: &'static str = "content";
        }

        impl AsRef<str> for Field {
            fn as_ref(&self) -> &str {
                match self {
                    Self::MetaData => Self::META_DATA,
                    Self::Content => Self::CONTENT,
                }
            }
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str("`metadata` or `content`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "metadata" => Ok(Field::MetaData),
                            "content" => Ok(Field::Content),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct EnvelopeVisitor<T0> {
            marker: PhantomData<T0>,
        }

        impl<T0> EnvelopeVisitor<T0> {
            pub const fn new() -> Self {
                Self { marker: PhantomData }
            }
        }

        impl<'de, T0> de::Visitor<'de> for EnvelopeVisitor<T0>
        where
            T0: de::DeserializeOwned + Label + Send,
        {
            type Value = Envelope<T0>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let t_name = std::any::type_name::<T0>();
                f.write_str(format!("struct Envelope<{}>", t_name).as_str())
            }

            fn visit_map<V>(self, mut map: V) -> Result<Envelope<T0>, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut metadata = None;
                let mut content = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::MetaData => {
                            if metadata.is_some() {
                                return Err(de::Error::duplicate_field(Field::META_DATA));
                            }
                            metadata = Some(map.next_value()?);
                        },
                        Field::Content => {
                            if content.is_some() {
                                return Err(de::Error::duplicate_field(Field::CONTENT));
                            }
                            content = Some(map.next_value()?);
                        },
                    }
                }

                let metadata = metadata.ok_or_else(|| de::Error::missing_field(Field::META_DATA))?;
                let content = content.ok_or_else(|| de::Error::missing_field(Field::CONTENT))?;
                Ok(Envelope::from_parts(metadata, content))
            }
        }

        const FIELDS: &[&str] = &[Field::META_DATA, Field::CONTENT];
        deserializer.deserialize_struct("Envelope", FIELDS, EnvelopeVisitor::<T>::new())
    }
}
