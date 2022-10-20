mod label {
    use pretty_assertions::assert_eq;

    use crate::{Label, Labeling};

    #[test]
    fn test_unit_labeler() {
        let actual = <dyn Labeling>::summon::<()>();
        assert_eq!(actual.label().as_ref(), "");
    }

    #[test]
    fn test_i64_labeler() {
        let l = i64::labeler();
        let actual = l.label();
        assert_eq!(actual.as_ref(), "i64");
    }

    #[test]
    fn test_str_labeler() {
        let actual = <dyn Labeling>::summon::<&str>().label();
        assert_eq!(actual.as_ref(), "&str");
    }
}

mod labeling {
    use std::borrow::Cow;
    use std::marker::PhantomData;

    use pretty_assertions::assert_eq;

    use crate::{CustomLabeling, Labeling, MakeLabeling, NoLabeling};

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

mod snowflake {
    use serde_test::{assert_tokens, Token};

    use crate::{RealTimeGenerator, SnowflakeIdGenerator};

    #[test]
    fn test_snowflake_id_serde() {
        let gen = SnowflakeIdGenerator::<RealTimeGenerator>::default();
        let id = gen.next_id();
        let id_value: i64 = id.into();
        assert_tokens(&id, &[Token::I64(id_value)]);
    }
}
