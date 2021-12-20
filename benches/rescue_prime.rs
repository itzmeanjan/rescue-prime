extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_rescue_prime::{hash_elements, merge, prepare_ark1, prepare_ark2, prepare_mds};

fn bench_hash_elements(c: &mut Criterion) {
  let input: [u64; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
  let mds = prepare_mds();
  let ark1 = prepare_ark1();
  let ark2 = prepare_ark2();

  c.bench_function("hash_elements", |b| {
    b.iter(|| {
      hash_elements(
        black_box(&input),
        black_box(mds),
        black_box(ark1),
        black_box(ark2),
      )
    });
  });
}

fn bench_merge(c: &mut Criterion) {
  let input: [u64; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
  let mds = prepare_mds();
  let ark1 = prepare_ark1();
  let ark2 = prepare_ark2();

  c.bench_function("merge", |b| {
    b.iter(|| {
      merge(
        black_box(input),
        black_box(mds),
        black_box(ark1),
        black_box(ark2),
      )
    });
  });
}

criterion_group!(rescue_prime, bench_hash_elements, bench_merge);
criterion_main!(rescue_prime);
