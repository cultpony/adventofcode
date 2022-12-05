use criterion::{criterion_group, criterion_main, Criterion};

use aoc::aoc2022::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("aoc2022d1p1", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| day1::part1())
    });

    c.bench_function("aoc2022d1p2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| day1::part2())
    });

    c.bench_function("aoc2022d2p1", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| day2::part1())
    });
    c.bench_function("aoc2022d2p2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| day2::part2())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
