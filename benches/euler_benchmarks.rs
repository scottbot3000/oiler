#[macro_use]
extern crate criterion;

use criterion::Criterion;
use oiler::euler::*;

fn e1_bench(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| euler_1()));
}



criterion_group!(benches, e1_bench);
criterion_main!(benches);