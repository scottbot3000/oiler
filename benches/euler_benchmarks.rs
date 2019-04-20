#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;
use oiler::euler::*;

fn e1_bench(c: &mut Criterion) {
    let limit = black_box(1_000_000);
    c.bench_function("Euler 1", move |b| b.iter(|| euler_1_loop(limit)));
}

criterion_group!(benches, e1_bench);
criterion_main!(benches);