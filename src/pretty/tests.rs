use pretty_assertions::assert_eq;

use crate::{AlphabetCodec, CustomLabeling, IdPrettifier, Label, LazyGenerator, PrettyIdGenerator};

struct NonLabelZed;

struct Foo;
impl Label for Foo {
    type Labeler = CustomLabeling;

    fn labeler() -> Self::Labeler {
        CustomLabeling::new("MyFooferNut")
    }
}

#[test]
fn test_non_label_custom_generator() {
    let gen: PrettyIdGenerator<NonLabelZed, CustomLabeling, LazyGenerator, AlphabetCodec> =
        PrettyIdGenerator::single_node_labeling(CustomLabeling::new("Zedster"), IdPrettifier::default());

    let actual = gen.next_id();
    assert_eq!(format!("{actual:?}"), format!("Zedster::{}", actual.pretty()))
}

#[test]
fn test_labeled_generator() {
    let gen: PrettyIdGenerator<Foo, CustomLabeling, LazyGenerator, AlphabetCodec> =
        PrettyIdGenerator::single_node(IdPrettifier::default());

    let actual = gen.next_id();
    assert_eq!(format!("{actual:?}"), format!("MyFooferNut::{}", actual.pretty()))
}

mod codec {
    use once_cell::sync::Lazy;
    use pretty_assertions::assert_eq;

    use crate::pretty::Codec;
    use crate::AlphabetCodec;

    static CODEC: Lazy<AlphabetCodec> = Lazy::new(|| AlphabetCodec::default());

    #[test]
    fn test_encode_value() {
        assert_eq!(CODEC.encode(23), "BA".to_string());
        assert_eq!(CODEC.encode(529), "BAA".to_string());
        assert_eq!(CODEC.encode(12167), "BAAA".to_string());
    }

    #[test]
    fn test_decode_value() {
        assert_eq!(CODEC.decode("BA"), 23);
        assert_eq!(CODEC.decode("ABA"), 23);
        assert_eq!(CODEC.decode("BAA"), 529);
        assert_eq!(CODEC.decode("BAB"), 530);
        assert_eq!(CODEC.decode("BAAA"), 12167);
        assert_eq!(CODEC.decode("HAPK"), 85477);
        assert_eq!(CODEC.decode("HPJD"), 92233);
    }
}

mod damm {
    use pretty_assertions::assert_eq;

    use crate::pretty::damm;

    #[test]
    fn test_calculate_check_digit() {
        let actual = damm::encode("572");
        assert_eq!(&actual, "5724");
        let actual = damm::encode("43881234567");
        assert_eq!(&actual, "438812345679");
        let with_checksum = damm::encode(&format!("{}", i64::MAX));
        assert_eq!(damm::is_valid(&with_checksum), true);
    }

    #[test]
    fn test_fail_on_checking_check_digit() {
        let with_checksum = damm::encode(&format!("{}", i64::MAX));

        for i in 0..with_checksum.len() {
            let mut sb_bytes: Vec<u8> = with_checksum.as_bytes().iter().copied().collect();
            let old_char = sb_bytes[i];
            let new_char = 47 + ((old_char + 1) % 10);
            // println!(
            //     "old_char:[{}]:[{}] new_char:[{}]:[{}]",
            //     old_char as char, old_char, new_char as char, new_char
            // );
            let item = &mut sb_bytes[i];
            *item = new_char;
            let corrupted = String::from_utf8_lossy(sb_bytes.as_slice());
            // println!("orig:[{}]\ncorr:[{}]\n", with_checksum, corrupted);
            assert_eq!(damm::is_valid(corrupted.as_ref()), false);
        }
    }
}

mod id {
    use pretty_assertions::assert_eq;

    use crate::{AlphabetCodec, Id, IdPrettifier, Label, LabeledRealtimeIdGenerator, MakeLabeling};

    struct Foo;
    impl Label for Foo {
        type Labeler = MakeLabeling<Self>;

        fn labeler() -> Self::Labeler {
            MakeLabeling::default()
        }
    }

    fn make_generator<T: Label>() -> LabeledRealtimeIdGenerator<T> {
        crate::PrettyIdGenerator::single_node(IdPrettifier::<AlphabetCodec>::default())
    }

    #[test]
    fn test_partial_ord() {
        let generator = make_generator::<()>();
        let a = generator.next_id();
        let b = generator.next_id();
        assert!(a < b);
    }

    #[test]
    fn test_display() {
        let generator = make_generator();
        let a: Id<Foo> = generator.next_id();
        assert_eq!(format!("{a}"), format!("Foo::{}", a.pretty()));
    }

    #[test]
    fn test_alternate_display() {
        let generator = make_generator();
        let a: Id<i64> = generator.next_id();
        assert_eq!(format!("{a:#}"), a.num().to_string());
    }

    #[test]
    fn test_debug() {
        let generator = make_generator();
        let a: Id<Foo> = generator.next_id();
        assert_eq!(format!("{a:?}"), format!("Foo::{}", a.pretty()));
    }

    #[test]
    fn test_alternate_debug() {
        let generator = make_generator();
        let a: Id<Foo> = generator.next_id();
        assert_eq!(
            format!("{a:#?}"),
            format!(
                "Id {{\n    label: \"{}\",\n    snowflake: Id({}),\n    pretty: \"{}\",\n}}",
                a.label(),
                a.num(),
                a.pretty()
            )
        );
    }

    #[test]
    fn test_id_cross_conversion() {
        let generator = make_generator();
        let a: Id<String> = generator.next_id();
        let before = format!("{:?}", a);
        assert_eq!(format!("String::{}", a.pretty()), before);

        let b: Id<usize> = a.relabel();
        let after = format!("{:?}", b);
        assert_eq!(format!("usize::{}", b.pretty()), after);
    }
}

mod prettifier {
    use claim::*;
    use itertools::Itertools;
    use once_cell::sync::Lazy;
    use pretty_assertions::assert_eq;
    use rand::distributions::Distribution;

    use crate::snowflake::Id as SnowflakeId;
    use crate::{
        Alphabet, AlphabetCodec, Generator, IdPrettifier, LazyGenerator, RealTimeGenerator, SnowflakeIdGenerator,
    };

    const EXAMPLE_ID: Lazy<SnowflakeId> = Lazy::new(|| 824227036833910784.into());

    #[test]
    fn test_generate_pretty_ids_with_leading_zeros() {
        let default = IdPrettifier::<AlphabetCodec>::default();
        println!("### default: {:?}", default);

        let max_pretty_id = default.prettify(i64::MAX);
        assert_eq!(&max_pretty_id, "HPJD-72036-HAPK-58077");

        let example_pretty_id = default.prettify(*EXAMPLE_ID);
        assert_eq!(&example_pretty_id, "ARPJ-27036-GVQS-07849");
        assert_eq!(&default.prettify(1), "AAAA-00000-AAAA-00013");

        let prettifier_by_8 = IdPrettifier {
            // encoder: AlphabetCodec::new(Alphabet::new("
            // ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")),
            parts_size: 8,
            leading_zeros: true,
            ..default
        };
        println!("### prettifier_by_8: {:?}", prettifier_by_8);
        assert_eq!(&prettifier_by_8.prettify(1), "00000000-AAAA-00000013");
        assert_eq!(&prettifier_by_8.prettify(i64::MAX), "00009223-FTYTHN-47758077");
    }

    #[test]
    fn test_generate_pretty_ids_without_leading_zeros() {
        let prettifier = IdPrettifier {
            leading_zeros: false,
            ..IdPrettifier::<AlphabetCodec>::default()
        };
        let max_pretty_id = prettifier.prettify(i64::MAX);
        assert_eq!(&max_pretty_id, "HPJD-72036-HAPK-58077");

        let example_pretty_id = prettifier.prettify(*EXAMPLE_ID);
        assert_eq!(&example_pretty_id, "RPJ-27036-GVQS-07849");

        assert_eq!(&prettifier.prettify(1), "13");

        let prettifier_by_8 = IdPrettifier { parts_size: 8, ..prettifier };
        assert_eq!(&prettifier_by_8.prettify(1), "13");
        assert_eq!(&prettifier_by_8.prettify(i64::MAX), "9223-FTYTHN-47758077");
    }

    #[test]
    fn test_find_seed_of_pretty_id_with_leading_zeros() {
        let prettified_with_leading = IdPrettifier::<AlphabetCodec>::default();
        // let max_pretty_id = prettified_with_leading.prettify(i64::MAX);
        // let example_pretty_id = prettified_with_leading.prettify(EXAMPLE_ID);

        assert_eq!(
            assert_ok!(prettified_with_leading.to_id_seed("HPJD-72036-HAPK-58077")),
            i64::MAX.into()
        );
        assert_eq!(
            assert_ok!(prettified_with_leading.to_id_seed("ARPJ-27036-GVQS-07849")),
            *EXAMPLE_ID
        );
        assert_eq!(
            assert_ok!(prettified_with_leading.to_id_seed("AAAA-00000-AAAA-00013")),
            1.into()
        );
    }

    #[test]
    fn test_find_seed_of_pretty_id_without_leading_zeros() {
        let prettified_without_leading_zeros = IdPrettifier {
            leading_zeros: false,
            ..IdPrettifier::<AlphabetCodec>::default()
        };
        // assert_eq!(assert_ok!(prettified_without_leading_zeros.to_id_seed("HPJD-72036-HAPK-58077")),
        // i64::MAX); assert_eq!(assert_ok!(prettified_without_leading_zeros.to_id_seed("
        // RPJ-27036-GVQS-07849")), EXAMPLE_ID);
        assert_eq!(assert_ok!(prettified_without_leading_zeros.to_id_seed("13")), 1.into());
    }

    #[test]
    fn test_validate_pretty_ids() {
        let prettifier = IdPrettifier::<AlphabetCodec>::default();
        assert!(prettifier.is_valid("HPJD-72036-HAPK-58077"));
        assert!(prettifier.is_valid("HPJD-72036-HAPK-58077"));
        assert!(prettifier.is_valid("ARPJ-27036-GVQS-07849"));
        assert!(!prettifier.is_valid("ARPJ-27036-GVQS-07840"));
        assert!(!prettifier.is_valid("ARPJ-27036-GVQS-07489"));
        assert!(!prettifier.is_valid("ARPJ-27036-GVQZ-07489"));
    }

    #[test]
    fn test_preserve_id_monotonicity() {
        let real_time = SnowflakeIdGenerator::<RealTimeGenerator>::default();
        let generator = SnowflakeIdGenerator::<Generator>::default();
        let lazy = SnowflakeIdGenerator::<LazyGenerator>::default();
        let prettifier = IdPrettifier::<AlphabetCodec>::default();

        let mut real_time_actual: Vec<String> = (1..=100)
            .into_iter()
            .map(|_| prettifier.prettify(real_time.next_id()))
            .collect();
        let mut real_time_expected: Vec<String> = real_time_actual.clone().into_iter().sorted().collect();
        assert_eq!(real_time_actual, real_time_expected);
        real_time_actual.reverse();
        real_time_expected.reverse();
        assert_eq!(real_time_actual, real_time_expected);

        let mut generator_actual: Vec<String> = (1..=100)
            .into_iter()
            .map(|_| prettifier.prettify(generator.next_id()))
            .collect();
        let mut generator_expected: Vec<String> = generator_actual.clone().into_iter().sorted().collect();
        assert_eq!(generator_actual, generator_expected);
        generator_actual.reverse();
        generator_expected.reverse();
        assert_eq!(generator_actual, generator_expected);

        let mut lazy_actual: Vec<String> = (1..=100)
            .into_iter()
            .map(|_| prettifier.prettify(lazy.next_id()))
            .collect();
        let mut lazy_expected: Vec<String> = lazy_actual.clone().into_iter().sorted().collect();
        assert_eq!(lazy_actual, lazy_expected);
        lazy_actual.reverse();
        lazy_expected.reverse();
        assert_eq!(lazy_actual, lazy_expected);
    }

    #[test]
    fn test_keep_same_id_length() {
        let prettifier = IdPrettifier::<AlphabetCodec>::default();
        let min_id = prettifier.prettify(0);
        let max_id = prettifier.prettify(i64::MAX);
        assert_eq!(min_id.len(), max_id.len());
    }

    #[test]
    fn test_calculate_seed_properly_with_default_settings() {
        let id_generator = SnowflakeIdGenerator::<RealTimeGenerator>::default();
        let prettifier = IdPrettifier::<AlphabetCodec>::default();
        (1..=10_000).into_iter().for_each(|_| {
            let seed = id_generator.next_id();
            let id = prettifier.prettify(seed);
            assert_eq!(assert_ok!(prettifier.to_id_seed(&id)), seed);
        });
    }

    #[test]
    fn test_calculate_seed_properly_without_leading_zeros() {
        let id_generator = SnowflakeIdGenerator::<RealTimeGenerator>::default();
        let prettifier = IdPrettifier {
            leading_zeros: false,
            ..IdPrettifier::<AlphabetCodec>::default()
        };

        (0..10_000).into_iter().for_each(|_| {
            let seed = id_generator.next_id();
            let id = prettifier.prettify(seed);
            assert_eq!(assert_ok!(prettifier.to_id_seed(&id)), seed);
        });

        let between = rand::distributions::Uniform::from(0..=i64::MAX);
        let mut rng = rand::thread_rng();

        (0..10_000).into_iter().for_each(|_| {
            let seed = between.sample(&mut rng);
            let id = prettifier.prettify(seed);
            let decoded_seed = assert_ok!(prettifier.to_id_seed(&id));
            assert_eq!(decoded_seed, seed.into());
        })
    }

    #[test]
    fn test_calculate_seed_properly_smaller_parts_and_short_alphabet() {
        let id_generator = SnowflakeIdGenerator::<RealTimeGenerator>::default();
        let prettifier = IdPrettifier {
            encoder: AlphabetCodec::new(Alphabet::new("ABC")),
            parts_size: 2,
            ..IdPrettifier::<AlphabetCodec>::default()
        };

        (0..10_000).into_iter().for_each(|_| {
            let seed = id_generator.next_id();
            let id = prettifier.prettify(seed);
            let decoded_seed = assert_ok!(prettifier.to_id_seed(&id));
            assert_eq!(decoded_seed, seed);
        });

        println!("#########");

        let between = rand::distributions::Uniform::from(0..=i64::MAX);
        let mut rng = rand::thread_rng();
        (0..10_000).into_iter().for_each(|_| {
            let seed = between.sample(&mut rng);
            let id = prettifier.prettify(seed);
            let decoded_seed = assert_ok!(prettifier.to_id_seed(&id));
            assert_eq!(decoded_seed, seed.into());
        })
    }
}
