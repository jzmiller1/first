use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate first;
use first::utils;


fn bench_frequency_O(c: &mut Criterion) {
    let mut group = c.benchmark_group("frequency");
    for size in [100, 1_000, 10_000].iter() {
        let s: String = std::iter::repeat("ABCD").take(*size / 4).collect();
        group.bench_with_input(criterion::BenchmarkId::new("Size", size), size, |b, &_| {
            b.iter(|| utils::frequency(black_box(&s)))
        });
    }
    group.finish();
}

fn bench_freq_to_prob_O(c: &mut Criterion) {
    let mut group = c.benchmark_group("freq_to_prob");
    for size in [100, 1_000, 10_000].iter() {
        let mut freq_map = HashMap::new();
        for ch in "ABCD".chars() {
            freq_map.insert(ch, *size / 4);
        }
        group.bench_with_input(criterion::BenchmarkId::new("Size", size), size, |b, &_| {
            b.iter(|| utils::freq_to_prob(black_box(&freq_map)))
        });
    }
    group.finish();
}

fn bench_entropy_O(c: &mut Criterion) {
    let mut group = c.benchmark_group("entropy");
    for size in [100, 1_000, 10_000].iter() {
        let mut freq_map = HashMap::new();
        for ch in "ABCD".chars() {
            freq_map.insert(ch, *size / 4);
        }
        let prob_map = utils::freq_to_prob(&freq_map);
        group.bench_with_input(criterion::BenchmarkId::new("Size", size), size, |b, &_| {
            b.iter(|| utils::entropy(black_box(&prob_map)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_frequency_O, bench_freq_to_prob_O, bench_entropy_O);
criterion_main!(benches);
