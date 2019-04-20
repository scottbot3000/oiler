#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::Bencher;
use criterion::black_box;
use criterion::Fun;
use oiler::euler::*;

fn bench_e1_loop(b: &mut Bencher, i: &u32) {
    b.iter(|| {
        euler_1(*i);
    });
}

fn bench_e1_iter(b: &mut Bencher, i: &u32) {
    b.iter(|| {
        euler_1_iter(*i);
    });
}

fn bench_e1_par_iter(b: &mut Bencher, i: &u32) {
    b.iter(|| {
        euler_1_par_iter(*i);
    });
}

fn bench_e1_loop_improved(b: &mut Bencher, i: &u32) {
    b.iter(|| {
        euler_1_loop(*i);
    });
}

fn e1_bench(c: &mut Criterion) {
    let limit = black_box(1_000_000);
    let for_loop = Fun::new("For Loop", bench_e1_loop);
    let iter = Fun::new("Iterator", bench_e1_iter);
    let par_iter = Fun::new("Parallel Iterator", bench_e1_par_iter);
    let for_loop_improved = Fun::new("For Loop Unrolled", bench_e1_loop_improved);
    let functions = vec![for_loop, iter, par_iter, for_loop_improved];

    c.bench_functions("Euler 1", functions, limit);
}

criterion_group!(benches, e1_bench);
criterion_main!(benches);