#![feature(test)]
extern crate test;

use pretty_snowflake::{
    AlphabetCodec, Generator, LazyGenerator, PrettyIdGenerator, RealTimeGenerator, SnowflakeIdGenerator,
};
use test::Bencher;

#[bench]
fn bench_generate_real_time_snowflake(b: &mut Bencher) {
    let mut generator = SnowflakeIdGenerator::<RealTimeGenerator>::default();
    b.iter(|| generator.next_id());
}

#[bench]
fn bench_generate_real_time_pretty(b: &mut Bencher) {
    let mut generator = PrettyIdGenerator::<RealTimeGenerator, AlphabetCodec>::default();
    b.iter(|| generator.next_id());
}

#[bench]
fn bench_generate_generator_snowflake(b: &mut Bencher) {
    let mut generator = SnowflakeIdGenerator::<Generator>::default();
    b.iter(|| generator.next_id());
}

#[bench]
fn bench_generate_generator_pretty(b: &mut Bencher) {
    let mut generator = PrettyIdGenerator::<Generator, AlphabetCodec>::default();
    b.iter(|| generator.next_id());
}

#[bench]
fn bench_generate_lazy_snowflake(b: &mut Bencher) {
    let mut generator = SnowflakeIdGenerator::<LazyGenerator>::default();
    b.iter(|| generator.next_id());
}

#[bench]
fn bench_generate_lazy_pretty(b: &mut Bencher) {
    let mut generator = PrettyIdGenerator::<LazyGenerator, AlphabetCodec>::default();
    b.iter(|| generator.next_id());
}
