use criterion::{criterion_group, criterion_main, Criterion};
use kvs::KvStore;

fn kvs_set() {
    let mut store = KvStore::new();
    for i in 0..=1_000 {
        store.set(format!("{}", i), String::from("bench"));
    }
}

fn kvs_set_bench(c: &mut Criterion) {
    c.bench_function("kvs set", |b| b.iter(|| kvs_set()));
}

criterion_group!(benches, kvs_set_bench);
criterion_main!(benches);
