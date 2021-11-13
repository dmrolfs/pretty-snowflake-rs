use std::borrow::Cow;
use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;

use pretty_type_name::pretty_type_name;

use crate::Label;

pub trait Labeling: Debug {
    fn label(&self) -> Cow<'static, str>;
}

impl dyn Labeling {
    pub fn summon<T: Label>() -> <T as Label>::Labeler {
        T::labeler()
    }
}

#[derive(Copy)]
pub struct MakeLabeling<T: ?Sized> {
    marker: PhantomData<T>,
}

#[derive(Clone)]
pub struct CustomLabeling {
    label: String,
}

#[derive(Copy, Clone)]
pub struct NoLabeling;

impl<T: ?Sized> MakeLabeling<T> {
    pub fn new() -> Self {
        Self { marker: PhantomData }
    }
}

impl<T: ?Sized> Default for MakeLabeling<T> {
    fn default() -> Self {
        MakeLabeling::new()
    }
}

impl<T: ?Sized> Labeling for MakeLabeling<T> {
    fn label(&self) -> Cow<'static, str> {
        Cow::Owned(pretty_type_name::<T>())
    }
}

impl<T: ?Sized> fmt::Debug for MakeLabeling<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MakeLabeling").field(&self.label()).finish()
    }
}

impl<T: ?Sized> fmt::Display for MakeLabeling<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl<T: ?Sized> Clone for MakeLabeling<T> {
    fn clone(&self) -> Self {
        Self { marker: PhantomData }
    }
}

impl CustomLabeling {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into() }
    }
}

impl Labeling for CustomLabeling {
    fn label(&self) -> Cow<'static, str> {
        Cow::Owned(self.label.to_owned())
    }
}

impl fmt::Debug for CustomLabeling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CustomLabeling").field(&self.label()).finish()
    }
}

impl fmt::Display for CustomLabeling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl From<&str> for CustomLabeling {
    fn from(that: &str) -> Self {
        Self { label: that.into() }
    }
}

impl From<String> for CustomLabeling {
    fn from(label: String) -> Self {
        Self { label }
    }
}

impl Labeling for NoLabeling {
    fn label(&self) -> Cow<'static, str> {
        Cow::Borrowed("")
    }
}

impl fmt::Debug for NoLabeling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EmptyLabeling").finish()
    }
}

impl fmt::Display for NoLabeling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    struct Foo;
    struct Zed;
    struct Bar<T> {
        marker: PhantomData<T>,
    }

    #[test]
    fn test_make_labeling() {
        let foo: MakeLabeling<Foo> = MakeLabeling::default();
        let bar_foo: MakeLabeling<Bar<Foo>> = MakeLabeling::default();
        let bar_zed: MakeLabeling<Bar<Zed>> = MakeLabeling::default();

        assert_ne!(foo.label(), bar_foo.label());
        assert_ne!(bar_zed.label(), bar_foo.label());
        assert_eq!(foo.label(), Cow::Borrowed("Foo"));
        assert_eq!(bar_foo.label(), Cow::Borrowed("Bar<Foo>"));
        assert_eq!(bar_zed.label(), Cow::Borrowed("Bar<Zed>"));
    }

    #[test]
    fn test_custom_labeling() {
        let foo = CustomLabeling::new("Foo");
        let bar = CustomLabeling::new("Bar");

        assert_ne!(foo.label(), bar.label());
        assert_eq!(foo.label(), Cow::Borrowed("Foo"));
        assert_eq!(bar.label(), Cow::Borrowed("Bar"));
    }

    #[test]
    fn test_empty_labeling() {
        assert_eq!(NoLabeling.label(), Cow::<'static, str>::default());
        assert_eq!(NoLabeling.label(), NoLabeling.label());
    }

    #[test]
    fn test_clone_make_labeling() {
        let expected: MakeLabeling<Foo> = MakeLabeling::default();
        let actual = expected.clone();
        assert_eq!(actual.label(), expected.label())
    }
}
