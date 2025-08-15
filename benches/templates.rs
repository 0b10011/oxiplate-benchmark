use criterion::{criterion_group, criterion_main};
use oxiplate_benchmark::run;

criterion_main!(benches);
criterion_group!(benches, run);
