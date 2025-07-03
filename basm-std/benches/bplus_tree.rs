use criterion::{Criterion, black_box, criterion_group, criterion_main};

use rand::SeedableRng;

use basm_std::collections::{BPTreeMap, LazyOp};
use basm_std::math;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("boj_16124", |b| {
        b.iter(|| {
            const P: u32 = 998_244_353;
            const N: usize = 1_000_000;
            const Q: usize = 100_000;

            use rand::Rng;
            let mut rng = rand::rngs::SmallRng::seed_from_u64(123);

            struct V([u32; 10], u32); // (values, power_of_10)
            struct U([u8; 10]); // lazy; .0[from] = to
            struct F;
            impl LazyOp<V, U> for F {
                fn binary_op(t1: &V, t2: &V) -> V {
                    let mut v = [0; 10];
                    for (i, v) in v.iter_mut().enumerate() {
                        *v = math::modadd(math::modmul(t1.0[i], t2.1, P), t2.0[i], P);
                    }
                    V(v, math::modmul(t1.1, t2.1, P))
                }
                fn apply(u: &U, t: &V) -> V {
                    let mut v = [0; 10];
                    for i in 0..10 {
                        let j = u.0[i] as usize;
                        v[j] = math::modadd(v[j], t.0[i], P);
                    }
                    V(v, t.1)
                }
                fn compose(u1: &U, u2: &U) -> U {
                    let mut u = [0u8; 10];
                    for (i, u) in u.iter_mut().enumerate() {
                        *u = u1.0[u2.0[i] as usize];
                    }
                    U(u)
                }
                fn id_op() -> U {
                    U([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
                }
            }
            let mut tree = BPTreeMap::<usize, V, U, F>::new();
            for i in 1..=N {
                let word = rng.random_range(0..10);
                let mut p = [0; 10];
                p[word] = 1;
                tree.insert(i, V(p, 10));
            }
            for _ in 0..Q {
                let kind = rng.random_range(1..=2);
                match kind {
                    1 => {
                        let (l, r) = {
                            let x = rng.random_range(1..=N);
                            let y = rng.random_range(1..=N);
                            if x <= y { (x, y) } else { (y, x) }
                        };
                        let from = rng.random_range(0..10);
                        let to = rng.random_range(0..10);
                        let mut range = tree.get_range_mut(l..=r).unwrap();
                        let mut u = F::id_op();
                        u.0[from] = to as u8;
                        range.apply(&u);
                    }
                    2 => {
                        let (l, r) = {
                            let x = rng.random_range(1..=N);
                            let y = rng.random_range(1..=N);
                            if x <= y { (x, y) } else { (y, x) }
                        };
                        let range = tree.get_range(l..=r).unwrap();
                        let mut ans = 0;
                        for i in 0..10 {
                            ans = math::modadd(ans, math::modmul(range.0[i], i as u32, P), P);
                        }
                        black_box(ans);
                    }
                    _ => unreachable!(),
                }
            }
        })
    });
}

criterion_group!(
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = criterion_benchmark
);
criterion_main!(benches);
