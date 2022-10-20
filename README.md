# pretty-snowflake

Rust port of [SoftwareMill/scala-id-generator](https://github.com/softwaremill/scala-id-generator/blob/master/README.md).

Generate unique ids. A default generator is provided, based on 
[Twitter Snowflake](https://github.com/twitter/snowflake), which generates time-based ids. 
Besides that library provide `IdPrettifier` which may convert `i64` into user-friendly Id type that 
can be coerced into an i64 for systematic usage and storage and as a human-readable string, such as 
`HPJD-72036-HAPK-58077`. 

`IdPrettifier` preserves the id's monotonicity, provides checksum and produces id with constant 
length (if it's not configured otherwise). It also maybe configured to user custom part sizes, 
separator or don't use leading zeros to provide fixed length. 

Examples
```rust
    // create instance of it
    let generator = PrettyIdGenerator::<RealTimeGenerator, AlphabetCodec>::single_node(IdPrettifier::default());

    // generate ids
    let actual = generator.next_id();
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
    assert_eq!(origin, 100);

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
```
