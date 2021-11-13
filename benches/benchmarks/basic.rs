// #![feature(test)]
// extern crate test;

// extern crate pretty_snowflake_derive;
// use pretty_snowflake_derive::*;

use criterion::{black_box, criterion_group, Criterion};
use pretty_snowflake::{
    AlphabetCodec, Generator, Label, LabeledBasicIdGenerator, LabeledLazyIdGenerator, LabeledRealtimeIdGenerator,
    LazyGenerator, MakeLabeling, PrettyIdGenerator, RealTimeGenerator, SnowflakeIdGenerator,
};

#[derive(Label)]
struct Foo;

fn bench_generate_real_time_snowflake(c: &mut Criterion) {
    c.bench_function("real_time_snowflake", move |b| {
        let mut generator = SnowflakeIdGenerator::<RealTimeGenerator>::default();
        b.iter(|| generator.next_id())
    });
}

fn bench_generate_real_time_pretty(c: &mut Criterion) {
    c.bench_function("real_time_pretty", move |b| {
        let mut generator = LabeledRealtimeIdGenerator::<Foo>::default();
        b.iter(|| generator.next_id())
    });
}

fn bench_generate_generator_snowflake(c: &mut Criterion) {
    c.bench_function("basic_snowflake", move |b| {
        let mut generator = SnowflakeIdGenerator::<Generator>::default();
        b.iter(|| generator.next_id())
    });
}

fn bench_generate_generator_pretty(c: &mut Criterion) {
    c.bench_function("basic_pretty", move |b| {
        let mut generator = LabeledBasicIdGenerator::<Foo>::default();
        b.iter(|| generator.next_id())
    });
}

fn bench_generate_lazy_snowflake(c: &mut Criterion) {
    c.bench_function("lazy_snowflake", move |b| {
        let mut generator = SnowflakeIdGenerator::<LazyGenerator>::default();
        b.iter(|| generator.next_id())
    });
}

fn bench_generate_lazy_pretty(c: &mut Criterion) {
    c.bench_function("lazy_pretty", move |b| {
        let mut generator = LabeledLazyIdGenerator::<Foo>::default();
        b.iter(|| generator.next_id())
    });
}

criterion_group! {
    name = basic;
    config = Criterion::default().with_profiler(super::super::profiler::FlamegraphProfiler::new(100));
    targets = 
        bench_generate_real_time_snowflake,
        bench_generate_generator_snowflake,
        bench_generate_lazy_snowflake,
        bench_generate_real_time_pretty,
        bench_generate_generator_pretty,
        bench_generate_lazy_pretty
}