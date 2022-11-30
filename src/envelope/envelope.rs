use crate::{Id, Label};
use iso8601_timestamp::Timestamp;
use std::fmt;
use std::future::Future;

use crate::envelope::{Correlation, MetaData, ReceivedAt};
#[cfg(feature = "functional")]
use frunk::{Monoid, Semigroup};

pub trait IntoEnvelope {
    type Content: Label;

    fn into_envelope(self) -> Envelope<Self::Content>;
    fn metadata(&self) -> MetaData<Self::Content>;
}

/// A metadata wrapper for a data set
#[derive(Clone)]
pub struct Envelope<T>
where
    T: Label,
{
    metadata: MetaData<T>,
    content: T,
}

impl<T> fmt::Debug for Envelope<T>
where
    T: fmt::Debug + Label + Send,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("[{}]{{ {:?} }}", self.metadata, self.content))
    }
}

impl<T> fmt::Display for Envelope<T>
where
    T: fmt::Display + Label + Send,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]({})", self.metadata, self.content)
    }
}

impl<T> Envelope<T>
where
    T: Label + Send,
{
    /// Create a new enveloped data.
    pub fn new(content: T) -> Self {
        Self { metadata: MetaData::default(), content }
    }

    /// Directly create enveloped data with given metadata.
    pub const fn direct(content: T, metadata: MetaData<T>) -> Self {
        Self { metadata, content }
    }

    /// Get a reference to the sensor data metadata.
    pub const fn metadata(&self) -> &MetaData<T> {
        &self.metadata
    }

    /// Consumes self, returning the data item
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    pub fn into_inner(self) -> T {
        self.content
    }

    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    pub fn into_parts(self) -> (MetaData<T>, T) {
        (self.metadata, self.content)
    }

    #[inline]
    pub const fn from_parts(metadata: MetaData<T>, content: T) -> Self {
        Self { metadata, content }
    }

    pub fn adopt_metadata<U>(&mut self, new_metadata: MetaData<U>) -> MetaData<T>
    where
        U: Label,
    {
        let old_metadata = self.metadata.clone();
        self.metadata = new_metadata.relabel();
        old_metadata
    }

    pub fn map<F, U>(self, f: F) -> Envelope<U>
    where
        U: Label + Send,
        F: FnOnce(T) -> U,
    {
        let metadata = self.metadata.clone().relabel();
        Envelope { metadata, content: f(self.content) }
    }

    pub fn flat_map<F, U>(self, f: F) -> Envelope<U>
    where
        U: Label + Send,
        F: FnOnce(Self) -> U,
    {
        let metadata = self.metadata.clone().relabel();
        Envelope { metadata, content: f(self) }
    }

    pub async fn and_then<Op, Fut, U>(self, f: Op) -> Envelope<U>
    where
        U: Label + Send,
        Fut: Future<Output = U> + Send,
        Op: FnOnce(T) -> Fut + Send,
    {
        let metadata = self.metadata.clone().relabel();
        Envelope { metadata, content: f(self.content).await }
    }
}

impl<T> Correlation for Envelope<T>
where
    T: Label + Sync,
{
    type Correlated = T;

    fn correlation(&self) -> &Id<Self::Correlated> {
        self.metadata.correlation()
    }
}

impl<T> ReceivedAt for Envelope<T>
where
    T: Label,
{
    fn recv_timestamp(&self) -> Timestamp {
        self.metadata.recv_timestamp()
    }
}

impl<T> Label for Envelope<T>
where
    T: Label,
{
    type Labeler = <T as Label>::Labeler;

    fn labeler() -> Self::Labeler {
        <T as Label>::labeler()
    }
}

impl<T> std::ops::Add for Envelope<T>
where
    T: std::ops::Add<Output = T> + Label + Send,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_parts(self.metadata + rhs.metadata, self.content + rhs.content)
    }
}

#[cfg(feature = "functional")]
impl<T> Monoid for Envelope<T>
where
    T: Monoid + Label + Send,
{
    fn empty() -> Self {
        Self::from_parts(<MetaData<T> as Monoid>::empty(), <T as Monoid>::empty())
    }
}

#[cfg(feature = "functional")]
impl<T> Semigroup for Envelope<T>
where
    T: Semigroup + Label + Send,
{
    fn combine(&self, other: &Self) -> Self {
        Self::from_parts(
            self.metadata().combine(other.metadata()),
            self.content.combine(&other.content),
        )
    }
}

impl<T> IntoEnvelope for Envelope<T>
where
    T: Label,
{
    type Content = T;

    fn into_envelope(self) -> Envelope<Self::Content> {
        self
    }

    fn metadata(&self) -> MetaData<Self::Content> {
        self.metadata.clone()
    }
}

impl<T> std::ops::Deref for Envelope<T>
where
    T: Label,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<T> std::ops::DerefMut for Envelope<T>
where
    T: Label,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl<T> AsRef<T> for Envelope<T>
where
    T: Label,
{
    fn as_ref(&self) -> &T {
        &self.content
    }
}

impl<T> AsMut<T> for Envelope<T>
where
    T: Label,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.content
    }
}

impl<T> PartialEq for Envelope<T>
where
    T: Label + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl<T> PartialEq<T> for Envelope<T>
where
    T: Label + PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.content == other
    }
}

impl<T> Envelope<Option<T>>
where
    T: Label + Send,
{
    /// Transposes an `Envelope` of an [`Option`] into an [`Option`] of `Envelope`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pretty_snowflake::envelope::Envelope;
    ///
    /// let x: Option<Envelope<i32>> = Some(Envelope::new(5));
    /// let y: Envelope<Option<i32>> = Envelope::new(Some(5));
    /// assert_eq!(x, y.transpose());
    /// ```
    #[inline]
    pub fn transpose(self) -> Option<Envelope<T>> {
        match self.content {
            Some(d) => Some(Envelope { content: d, metadata: self.metadata.relabel() }),
            None => None,
        }
    }
}

impl<T, E> Envelope<Result<T, E>>
where
    T: Label + Send,
{
    /// Transposes a `Envelope` of a [`Result`] into a [`Result`] of `Envelope`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pretty_snowflake::envelope::Envelope;
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct SomeErr;
    ///
    /// let x: Result<Envelope<i32>, SomeErr> = Ok(Envelope::new(5));
    /// let y: Envelope<Result<i32, SomeErr>> = Envelope::new(Ok(5));
    /// assert_eq!(x, y.transpose());
    /// ```
    #[inline]
    pub fn transpose(self) -> Result<Envelope<T>, E> {
        match self.content {
            Ok(content) => Ok(Envelope { content, metadata: self.metadata.relabel() }),
            Err(e) => Err(e),
        }
    }
}
