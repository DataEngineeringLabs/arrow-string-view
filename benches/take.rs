use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use rand::distributions::Alphanumeric;
use rand::{rngs::StdRng, Rng, SeedableRng};

use arrow2::array::{Offset, Utf8Array};

use string_view::*;

fn create_random_index(size: usize) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(42);
    (0..size)
        .map(|_| rng.gen_range::<u64, _>(0..size as u64))
        .collect()
}

/// Creates an random (but fixed-seeded) [`Utf8Array`] of a given length, number of characters and null density.
pub fn create_array<O: Offset>(length: usize, size: usize, seed: u64) -> Utf8Array<O> {
    let mut rng = StdRng::seed_from_u64(seed);

    let iterator = (0..length).map(|_| {
        let size = rng.gen_range::<usize, _>(0..size);

        (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(size)
            .map(char::from)
            .collect::<String>()
    });
    Utf8Array::<O>::from_iter_values(iterator)
}

/// Creates an random (but fixed-seeded) [`Utf8Array`] of a given length, number of characters and null density.
pub fn create_view(length: usize, size: usize, seed: u64) -> StringView {
    let mut rng = StdRng::seed_from_u64(seed);

    let iterator = (0..length).map(|_| {
        let size = rng.gen_range::<usize, _>(0..size);

        (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(size)
            .map(char::from)
            .collect::<String>()
    });
    StringView::from_iterator(iterator)
}

fn add_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("take");
    for log2_size in (10..=20).step_by(2) {
        let size = 2usize.pow(log2_size);

        let view = create_view(size, 20, 0);
        let array = create_array::<i32>(size, 20, 0);
        let indices = create_random_index(size);

        group.bench_with_input(
            BenchmarkId::new("view", log2_size),
            &(&view, &indices),
            |b, (view, indices)| b.iter(|| take_view(view, indices)),
        );

        group.bench_with_input(
            BenchmarkId::new("array", log2_size),
            &(&array, &indices),
            |b, (array, indices)| b.iter(|| take_array(array, indices)),
        );
    }
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
