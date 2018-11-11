#[macro_use]
extern crate criterion;
extern crate endo;

use criterion::Criterion;
use endo::DnaExecutor;

const ENDO_DNA: &'static str = include_str!("../data/endo.dna");

fn endo_decode(loops: usize) {
    let mut dna_executor = DnaExecutor::from(ENDO_DNA);
    dna_executor.execute_loops(loops);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("endo decode 1000", |b| b.iter(|| endo_decode(1000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);