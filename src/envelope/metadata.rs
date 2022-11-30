use crate::envelope::{Correlation, ReceivedAt};
use crate::{generator, Id, Label, Labeling};
use frunk::{Monoid, Semigroup};
use iso8601_timestamp::Timestamp;
use once_cell::sync::Lazy;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

use serde::{Deserialize, Serialize};

pub static CORRELATION_SNOWFLAKE_ID_KEY: Lazy<String> = Lazy::new(|| "correlation_snowflake_id".to_string());
pub static CORRELATION_PRETTY_ID_KEY: Lazy<String> = Lazy::new(|| "correlation_pretty_id".to_string());
pub static RECV_TIMESTAMP_KEY: Lazy<String> = Lazy::new(|| "recv_timestamp".to_string());

pub trait IntoMetaData {
    type CorrelatedType: Label;

    fn into_metadata(self) -> MetaData<Self::CorrelatedType>;
}

impl IntoMetaData for HashMap<String, String> {
    type CorrelatedType = ();

    fn into_metadata(mut self) -> MetaData<Self::CorrelatedType> {
        let _dropped = self.remove(CORRELATION_PRETTY_ID_KEY.deref());

        let correlation_id =
            self.remove(CORRELATION_SNOWFLAKE_ID_KEY.deref())
                .map_or_else(generator::next_id, |snowflake_rep| {
                    i64::from_str(snowflake_rep.as_str())
                        .map(|id| {
                            Id::new(
                                <Self::CorrelatedType as Label>::labeler().label(),
                                id,
                                &generator::prettifier(),
                            )
                        })
                        .unwrap_or_else(|_| generator::next_id())
                });

        let recv_timestamp = self
            .remove(RECV_TIMESTAMP_KEY.deref())
            .map_or_else(Timestamp::now_utc, |ts| {
                Timestamp::parse(ts.as_str()).unwrap_or_else(Timestamp::now_utc)
            });

        let custom = if !self.is_empty() { Some(self) } else { None };

        MetaData::from_parts(correlation_id, recv_timestamp, custom)
    }
}

/// A set of metdata regarding the envelope contents.
#[derive(Serialize, Deserialize)]
pub struct MetaData<T>
where
    T: Label,
{
    correlation_id: Id<T>,
    recv_timestamp: Timestamp,
    custom: HashMap<String, String>,
}

impl<T> fmt::Debug for MetaData<T>
where
    T: Label,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("MetaData");
        debug.field("correlation", &self.correlation_id);
        debug.field("recv_timestamp", &self.recv_timestamp.to_string());

        if !self.custom.is_empty() {
            debug.field("custom", &self.custom);
        }

        debug.finish()
    }
}

impl<T> fmt::Display for MetaData<T>
where
    T: Label + Send,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let custom_rep = format!("{:?}", self.custom);
        write!(f, "{} @ {}{}", self.correlation_id, self.recv_timestamp, custom_rep)
    }
}

impl<T> Default for MetaData<T>
where
    T: Label,
{
    fn default() -> Self {
        Self::from_parts(generator::next_id(), Timestamp::now_utc(), None)
    }
}

impl<T> MetaData<T>
where
    T: Label,
{
    pub fn from_parts(
        correlation_id: Id<T>, recv_timestamp: Timestamp, custom: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            correlation_id,
            recv_timestamp,
            custom: custom.unwrap_or_default(),
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn with_recv_timestamp(self, recv_timestamp: Timestamp) -> Self {
        Self { recv_timestamp, ..self }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn into_parts(self) -> (Id<T>, Timestamp, HashMap<String, String>) {
        (self.correlation_id, self.recv_timestamp, self.custom)
    }

    pub fn relabel<U: Label>(self) -> MetaData<U> {
        MetaData {
            correlation_id: self.correlation_id.relabel(),
            recv_timestamp: self.recv_timestamp,
            custom: self.custom,
        }
    }
}

impl<T> Correlation for MetaData<T>
where
    T: Label + Sync,
{
    type Correlated = T;

    fn correlation(&self) -> &Id<Self::Correlated> {
        &self.correlation_id
    }
}

impl<T> ReceivedAt for MetaData<T>
where
    T: Label,
{
    fn recv_timestamp(&self) -> Timestamp {
        self.recv_timestamp
    }
}

impl<T> Clone for MetaData<T>
where
    T: Label,
{
    fn clone(&self) -> Self {
        Self {
            correlation_id: self.correlation_id.clone(),
            recv_timestamp: self.recv_timestamp,
            custom: self.custom.clone(),
        }
    }
}

impl<T> PartialEq for MetaData<T>
where
    T: Label,
{
    fn eq(&self, other: &Self) -> bool {
        self.correlation_id == other.correlation_id
    }
}

impl<T> Eq for MetaData<T> where T: Label {}

impl<T> PartialOrd for MetaData<T>
where
    T: Label,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.recv_timestamp.partial_cmp(&other.recv_timestamp)
    }
}

impl<T> Ord for MetaData<T>
where
    T: Label,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.recv_timestamp.cmp(&other.recv_timestamp)
    }
}

impl<T> std::hash::Hash for MetaData<T>
where
    T: Label,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.correlation_id.hash(state)
    }
}

impl<T> std::ops::Add for MetaData<T>
where
    T: Label,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self < rhs {
            rhs
        } else {
            self
        }
    }
}

#[cfg(feature = "functional")]
impl<T> Monoid for MetaData<T>
where
    T: Label,
{
    fn empty() -> Self {
        Self::from_parts(generator::next_id(), Timestamp::UNIX_EPOCH, None)
    }
}

#[cfg(feature = "functional")]
impl<T> Semigroup for MetaData<T>
where
    T: Label,
{
    fn combine(&self, other: &Self) -> Self {
        if self < other {
            other.clone()
        } else {
            self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::envelope::Envelope;
    use crate::{Label, Labeling, MakeLabeling};
    use pretty_assertions::assert_eq;
    use serde_test::Configure;
    use serde_test::{assert_tokens, Token};

    const METADATA_TS: &str = "2022-11-30T03:43:18.068Z";

    static META_DATA: Lazy<MetaData<TestData>> = Lazy::new(|| {
        let ts = Timestamp::parse(METADATA_TS).unwrap();
        MetaData::default().with_recv_timestamp(ts)
    });

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestData(i32);

    impl Label for TestData {
        type Labeler = MakeLabeling<Self>;

        fn labeler() -> Self::Labeler {
            MakeLabeling::default()
        }
    }

    #[derive(Debug, PartialEq)]
    struct TestContainer(TestData);

    impl Label for TestContainer {
        type Labeler = MakeLabeling<Self>;

        fn labeler() -> Self::Labeler {
            MakeLabeling::default()
        }
    }

    #[derive(Debug, PartialEq)]
    struct TestEnvelopeContainer(Envelope<TestData>);

    impl Label for TestEnvelopeContainer {
        type Labeler = MakeLabeling<Self>;

        fn labeler() -> Self::Labeler {
            MakeLabeling::default()
        }
    }

    #[test]
    fn test_envelope_map() {
        let data = TestData(13);

        let metadata = MetaData::from_parts(
            Id::direct(<TestData as Label>::labeler().label(), 0, "zero"),
            Timestamp::now_utc(),
            None,
        );
        let enveloped_data = Envelope::from_parts(metadata.clone(), data);
        let expected = TestContainer(enveloped_data.clone().into_inner());
        let actual = enveloped_data.map(TestContainer);

        assert_eq!(actual.metadata().correlation().num(), metadata.correlation().num());
        assert_eq!(
            actual.metadata().correlation().pretty(),
            metadata.correlation().pretty()
        );
        assert_eq!(actual.metadata().recv_timestamp(), metadata.recv_timestamp());
        assert_eq!(actual.as_ref(), &expected);
    }

    #[test]
    fn test_envelope_flat_map() {
        let data = TestData(13);
        let mut custom = HashMap::default();
        custom.insert("cat".to_string(), "Otis".to_string());

        let metadata = MetaData::from_parts(
            Id::direct(<TestData as Label>::labeler().label(), 0, "zero"),
            Timestamp::now_utc(),
            Some(custom),
        );
        let enveloped_data = Envelope::from_parts(metadata.clone(), data);
        let expected = TestEnvelopeContainer(enveloped_data.clone());
        let actual = enveloped_data.flat_map(TestEnvelopeContainer);

        assert_eq!(actual.metadata().correlation().num(), metadata.correlation().num());
        assert_eq!(
            actual.metadata().correlation().pretty(),
            metadata.correlation().pretty()
        );
        assert_eq!(actual.metadata().recv_timestamp(), metadata.recv_timestamp());
        assert_eq!(actual.as_ref(), &expected);
    }

    #[test]
    fn test_envelope_serde_tokens() {
        let data = TestData(17);
        let actual = Envelope::from_parts(META_DATA.clone(), data);

        assert_tokens(
            &actual.readable(),
            &vec![
                Token::Struct { name: "Envelope", len: 2 },
                Token::Str("metadata"),
                Token::Struct { name: "MetaData", len: 3 },
                Token::Str("correlation_id"),
                Token::Struct { name: "Id", len: 2 },
                Token::Str("snowflake"),
                Token::I64(META_DATA.correlation_id.num()),
                Token::Str("pretty"),
                Token::Str(META_DATA.correlation_id.pretty()),
                Token::StructEnd,
                Token::Str("recv_timestamp"),
                Token::Str(METADATA_TS),
                Token::Str("custom"),
                Token::Map { len: Some(0) },
                Token::MapEnd,
                Token::StructEnd,
                Token::Str("content"),
                Token::NewtypeStruct { name: "TestData" },
                Token::I32(17),
                Token::StructEnd,
            ],
        )
    }
}
