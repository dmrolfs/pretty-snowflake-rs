use claim::*;
use pretty_assertions::assert_eq;
use pretty_snowflake::{
    Alphabet, AlphabetCodec, Id, IdPrettifier, Label, LabeledRealtimeIdGenerator, MakeLabeling, PrettyIdGenerator,
};
use regex::Regex;

#[derive(Label)]
struct Zed;

// impl Label for Zed {
//     type Labeler = MakeLabeling<Zed>;
//
//     fn labeler() -> Self::Labeler {
//         MakeLabeling::default()
//     }
// }

#[test]
fn test_present_example_of_usage() {
    // create instance of it
    let mut generator: LabeledRealtimeIdGenerator<Zed> = PrettyIdGenerator::single_node(IdPrettifier::default());

    // generate ids
    let actual: Id<Zed> = generator.next_id();
    let actual_str: String = actual.clone().into();
    assert!(!actual_str.is_empty());

    let re = Regex::new(r##"[A-Z]{4}-[0-9]{5}-[A-Z]{4}-[0-9]{5}"##).unwrap();
    assert!(re.is_match(&actual_str));
    // let expected = assert_ok!(IdPrettifier::<AlphabetCodec>::default().to_id_seed(&actual_str));
    // let actual_nr: i64 = actual.into();
    // assert_eq!(actual_nr, expected);

    // or it might be used just for encoding existing ids
    let prettifier = IdPrettifier::<AlphabetCodec>::default();
    let id = prettifier.prettify(100);
    assert_eq!(&id, "AAAA-00000-AAAA-01007");

    // get seed
    let origin = assert_ok!(prettifier.to_id_seed(&id));
    assert_eq!(origin, 100.into());

    // use custom prettifier
    let custom_prettifier = IdPrettifier {
        encoder: AlphabetCodec::new(Alphabet::new("ABC")),
        parts_size: 4,
        delimiter: '_'.to_string(),
        leading_zeros: false,
        ..IdPrettifier::default()
    };
    let custom_id = custom_prettifier.prettify(1234567);
    assert_eq!(&custom_id, "BCAACAB_5671");
}
