use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hint::black_box;
use std::time::Duration;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BatchSize;
use criterion::BenchmarkId;
use criterion::Criterion;
use indexmap::IndexMap;
use itertools::Itertools;
use more_collections::VecMap;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::thread_rng;

fn benchmark_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");
    group
        .sample_size(100)
        .measurement_time(Duration::from_millis(1000))
        .warm_up_time(Duration::from_millis(100));
    let test_insert_keys = vec![0, 50, 150];

    let empty = vec![];
    let half_full = (0usize..100)
        .step_by(2)
        .map(|i| (i, "hello".to_string()))
        .collect::<Vec<_>>();
    let mut almost_full = (0..100)
        .map(|i| (i, "hello".to_string()))
        .collect::<Vec<_>>();
    almost_full.retain(|(x, _)| !test_insert_keys.contains(x));

    let initial_states = vec![empty, half_full, almost_full];

    for initial_data in initial_states {
        for k in test_insert_keys.clone() {
            let parameter_string = format!("len:{:0>3}-key:{:0>3}", initial_data.len(), k);
            group.bench_with_input(
                BenchmarkId::new(parameter_string.clone(), "VecMap"),
                &k,
                |b, input| {
                    b.iter_batched_ref(
                        || VecMap::from_iter(initial_data.clone()),
                        |x| {
                            x.insert(*input, "new value".to_string());
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
            group.bench_with_input(
                BenchmarkId::new(parameter_string.clone(), "IndexMap"),
                &k,
                |b, input| {
                    b.iter_batched_ref(
                        || IndexMap::from_iter(initial_data.clone()),
                        |x: &mut IndexMap<usize, String>| {
                            x.insert(*input, "new value".to_string());
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
            group.bench_with_input(
                BenchmarkId::new(parameter_string.clone(), "HashMap"),
                &k,
                |b, input| {
                    b.iter_batched_ref(
                        || HashMap::from_iter(initial_data.clone()),
                        |x: &mut HashMap<usize, String>| {
                            x.insert(*input, "new value".to_string());
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
            group.bench_with_input(
                BenchmarkId::new(parameter_string.clone(), "BTreeMap"),
                &k,
                |b, input| {
                    b.iter_batched_ref(
                        || BTreeMap::from_iter(initial_data.clone()),
                        |x| {
                            x.insert(*input, "new value".to_string());
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
        }
    }
}

fn benchmark_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");
    group
        .sample_size(100)
        .measurement_time(Duration::from_millis(1000))
        .warm_up_time(Duration::from_millis(100));
    let test_insert_keys = vec![0, 50, 150];

    let initial_states = test_cases();

    for initial_data in initial_states {
        for k in test_insert_keys.clone() {
            let parameter_string = format!("{}-key:{:0>3}", initial_data.name, k);
            group.bench_with_input(
                BenchmarkId::new(parameter_string.clone(), "VecMap"),
                &k,
                |b, input| {
                    b.iter_batched_ref(
                        || VecMap::from_iter(initial_data.data.clone()),
                        |x| {
                            let res = x.get(*input);
                            black_box(res);
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
            group.bench_with_input(
                BenchmarkId::new(parameter_string.clone(), "IndexMap"),
                &k,
                |b, input| {
                    b.iter_batched_ref(
                        || IndexMap::from_iter(initial_data.data.clone()),
                        |x: &mut IndexMap<usize, String>| {
                            let res = x.get(input);
                            black_box(res);
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
            group.bench_with_input(
                BenchmarkId::new(parameter_string.clone(), "HashMap"),
                &k,
                |b, input| {
                    b.iter_batched_ref(
                        || HashMap::from_iter(initial_data.data.clone()),
                        |x: &mut HashMap<usize, String>| {
                            let res = x.get(input);
                            black_box(res);
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
            group.bench_with_input(
                BenchmarkId::new(parameter_string.clone(), "BTreeMap"),
                &k,
                |b, input| {
                    b.iter_batched_ref(
                        || BTreeMap::from_iter(initial_data.data.clone()),
                        |x| {
                            let res = x.get(input);
                            black_box(res);
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
        }
    }
}

fn benchmark_contains_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains_key");
    group
        .sample_size(100)
        .measurement_time(Duration::from_millis(1000))
        .warm_up_time(Duration::from_millis(100));
    let test_insert_keys = vec![0, 50, 150];

    let cases = test_cases();

    for case in cases {
        for k in test_insert_keys.clone() {
            let params = format!("{}-key:{:0>3}", case.name, k);
            group.bench_with_input(
                BenchmarkId::new(params.clone(), "VecMap"),
                &k,
                |b, input| {
                    b.iter_batched_ref(
                        || VecMap::from_iter(case.data.clone()),
                        |x| x.contains_key(*input),
                        BatchSize::SmallInput,
                    )
                },
            );
            group.bench_with_input(
                BenchmarkId::new(params.clone(), "IndexMap"),
                &k,
                |b, input| {
                    b.iter_batched_ref(
                        || IndexMap::from_iter(case.data.clone()),
                        |x: &mut IndexMap<usize, String>| x.contains_key(input),
                        BatchSize::SmallInput,
                    )
                },
            );
            group.bench_with_input(
                BenchmarkId::new(params.clone(), "HashMap"),
                &k,
                |b, input| {
                    b.iter_batched_ref(
                        || HashMap::from_iter(case.data.clone()),
                        |x: &mut HashMap<usize, String>| x.contains_key(input),
                        BatchSize::SmallInput,
                    )
                },
            );
            group.bench_with_input(
                BenchmarkId::new(params.clone(), "BTreeMap"),
                &k,
                |b, input| {
                    b.iter_batched_ref(
                        || BTreeMap::from_iter(case.data.clone()),
                        |x| x.contains_key(input),
                        BatchSize::SmallInput,
                    )
                },
            );
        }
    }
}

#[derive(Clone)]
struct TestCase {
    name: &'static str,
    data: Vec<(usize, String)>,
}

fn test_cases() -> [TestCase; 5] {
    [
        TestCase {
            name: "empty",
            data: vec![],
        },
        TestCase {
            name: "half_full",
            data: (0usize..100)
                .step_by(2)
                .map(|i| (i, "hello".to_string()))
                .collect::<Vec<_>>(),
        },
        TestCase {
            name: "almost_full",
            data: (0..100)
                .map(|i| (i, "hello".to_string()))
                .collect::<Vec<_>>(),
        },
        TestCase {
            name: "big_sparse",
            data: Uniform::new(0, 10_000)
                .sample_iter(thread_rng())
                .unique()
                .take(50)
                .map(|i| (i, "hello".to_string()))
                .collect::<Vec<_>>(),
        },
        TestCase {
            name: "big_dense",
            data: Uniform::new(0, 10_000)
                .sample_iter(thread_rng())
                .unique()
                .take(9800)
                .map(|i| (i, "hello".to_string()))
                .collect::<Vec<_>>(),
        },
    ]
}

fn benchmark_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter");
    group
        .sample_size(100)
        .measurement_time(Duration::from_millis(1000))
        .warm_up_time(Duration::from_millis(100));

    let initial_states = test_cases();

    for case in initial_states {
        let params = case.name;
        group.bench_function(BenchmarkId::new(params, "VecMap2"), |b| {
            b.iter_batched_ref(
                || VecMap::from_iter(case.data.clone()),
                |x| {
                    let result = x.iter2().collect::<Vec<_>>();
                    black_box(result);
                },
                BatchSize::SmallInput,
            )
        });
        bench_impl!(group, params, case, Vec, |x: &mut Vec<_>| {
            black_box(x.iter().collect::<Vec<_>>());
        });
        bench_impl!(group, params, case, VecMap, |x: &mut VecMap<_, _>| {
            black_box(x.iter().collect::<Vec<_>>());
        });
        bench_impl!(group, params, case, IndexMap, |x: &mut IndexMap<_, _>| {
            black_box(x.iter().collect::<Vec<_>>());
        });
        bench_impl!(group, params, case, HashMap, |x: &mut HashMap<_, _>| {
            black_box(x.iter().collect::<Vec<_>>());
        });
        bench_impl!(group, params, case, BTreeMap, |x| {
            black_box(x.iter().collect::<Vec<_>>());
        });
    }
}

criterion_group!(
    benches,
    benchmark_insert,
    benchmark_get,
    benchmark_contains_key,
    benchmark_iter
);
criterion_main!(benches);

#[macro_export]
macro_rules! bench_impl {
    ($group:ident, $param:ident, $test_case:expr, $Collection:tt, $code:expr ) => {
        $group.bench_function(BenchmarkId::new($param, stringify!($Collection)), |b| {
            b.iter_batched_ref(
                || $Collection::from_iter($test_case.data.clone()),
                $code,
                BatchSize::SmallInput,
            )
        });
    };
}
