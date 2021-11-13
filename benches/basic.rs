#![feature(test)]
extern crate test;

use pretty_snowflake::{
    AlphabetCodec, Generator, Label, LabeledBasicIdGenerator, LabeledLazyIdGenerator, LabeledRealtimeIdGenerator,
    LazyGenerator, MakeLabeling, PrettyIdGenerator, RealTimeGenerator, SnowflakeIdGenerator,
};
use test::Bencher;

struct Foo;

impl Label for Foo {
    type Labeler = MakeLabeling<Self>;

    fn labeler() -> Self::Labeler {
        MakeLabeling::default()
    }
}

#[bench]
fn bench_generate_real_time_snowflake(b: &mut Bencher) {
    let mut generator = SnowflakeIdGenerator::<RealTimeGenerator>::default();
    b.iter(|| generator.next_id());
}

#[bench]
fn bench_generate_real_time_pretty(b: &mut Bencher) {
    let mut generator = LabeledRealtimeIdGenerator::<Foo>::default();
    b.iter(|| generator.next_id());
}

#[bench]
fn bench_generate_generator_snowflake(b: &mut Bencher) {
    let mut generator = SnowflakeIdGenerator::<Generator>::default();
    b.iter(|| generator.next_id());
}

#[bench]
fn bench_generate_generator_pretty(b: &mut Bencher) {
    let mut generator = LabeledBasicIdGenerator::<Foo>::default();
    b.iter(|| generator.next_id());
}

#[bench]
fn bench_generate_lazy_snowflake(b: &mut Bencher) {
    let mut generator = SnowflakeIdGenerator::<LazyGenerator>::default();
    b.iter(|| generator.next_id());
}

#[bench]
fn bench_generate_lazy_pretty(b: &mut Bencher) {
    let mut generator = LabeledLazyIdGenerator::<Foo>::default();
    b.iter(|| generator.next_id());
}
