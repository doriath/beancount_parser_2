use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

#[cfg(feature = "unstable")]
use beancount_parser::parse;
use beancount_parser::Parser;

const SAMPLE: &str = include_str!("../tests/samples/official.beancount");

pub fn run_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse official example");
    group.significance_level(0.01);
    group.throughput(Throughput::Bytes(SAMPLE.len() as u64));
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("nom implementation", |b| {
        b.iter(|| Parser::new(SAMPLE).collect::<Result<Vec<_>, _>>().unwrap())
    });

    #[cfg(feature = "unstable")]
    group.bench_function("pest implementation", |b| {
        b.iter(|| parse(SAMPLE).unwrap().collect::<Vec<_>>())
    });

    group.finish();
}

criterion_group!(benches, run_bench);
criterion_main!(benches);
