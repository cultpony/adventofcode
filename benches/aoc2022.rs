use criterion::{criterion_group, criterion_main, Criterion};

use aoc::aoc2022::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("aoc2022d1p1", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day1::part1)
    });

    c.bench_function("aoc2022d1p2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day1::part2)
    });

    c.bench_function("aoc2022d2p1", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day2::part1)
    });
    c.bench_function("aoc2022d2p2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day2::part2)
    });

    c.bench_function("aoc2022d3p1", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day3::part1)
    });
    c.bench_function("aoc2022d3p2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day3::part2)
    });

    c.bench_function("aoc2022d4p1", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day4::part1)
    });
    c.bench_function("aoc2022d4p2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day4::part2)
    });

    c.bench_function("aoc2022d5p1", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day5::part1)
    });
    c.bench_function("aoc2022d5p2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day5::part2)
    });

    c.bench_function("aoc2022d6p1", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day6::part1)
    });
    c.bench_function("aoc2022d6p2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day6::part2)
    });

    c.bench_function("aoc2022d7p1", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day7::part1)
    });
    c.bench_function("aoc2022d7p2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day7::part2)
    });

    c.bench_function("aoc2022d8p1", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day8::part1)
    });
    c.bench_function("aoc2022d8p2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day8::part2)
    });

    c.bench_function("aoc2022d9p1", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day9::part1)
    });
    c.bench_function("aoc2022d9p2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day9::part2)
    });

    c.bench_function("aoc2022d10p1", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day10::part1)
    });
    c.bench_function("aoc2022d10p2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(day10::part2)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
