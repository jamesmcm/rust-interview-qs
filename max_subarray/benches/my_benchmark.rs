use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use max_subarray::Solution;
use rand::distributions::{Distribution, Uniform};

fn rand_vec(n: usize) -> Vec<i32> {
    let between = Uniform::from(-100..100);
    let mut rng = rand::thread_rng();
    between.sample_iter(&mut rng).take(n).collect()
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("MaxSubarray");
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(10));
    group.sample_size(10);

    let mut rand_arrays: Vec<Vec<i32>> = Vec::with_capacity(100);
    for i in [
        1, 2, 5, 10, 15, 20, 25, 30, 50, 75, 100, 150, 200, 300, 500, 750, 1000,
    ]
    .iter()
    {
        rand_arrays.push(rand_vec(*i));
    }

    for i in rand_arrays.iter() {
        group.bench_with_input(BenchmarkId::new("N", i.len()), i, |b, i| {
            b.iter(|| Solution::max_sub_array(i.clone()))
        });
        group.bench_with_input(BenchmarkId::new("N**2", i.len()), i, |b, i| {
            b.iter(|| Solution::max_sub_array_n2(i.clone()))
        });
        group.bench_with_input(BenchmarkId::new("N**3", i.len()), i, |b, i| {
            b.iter(|| Solution::max_sub_array_n3(i.clone()))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
