// benches/ip_store_bench.rs - Performance benchmarks for IP storage

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use std::net::IpAddr;
use std::str::FromStr;
use unique_ip_counter::storage::{IpStore, IpStoreImpl};

fn generate_ips(count: usize) -> Vec<IpAddr> {
    (1..=count)
        .map(|i| {
            let octet1 = (i / (256 * 256)) % 256;
            let octet2 = (i / 256) % 256;
            let octet3 = i % 256;
            IpAddr::from_str(&format!("10.{}.{}.{}", octet1, octet2, octet3)).unwrap()
        })
        .collect()
}

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("ip_store_insert");

    for size in [100, 1000, 10000].iter() {
        let ips = generate_ips(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let store = IpStoreImpl::new();
                for ip in &ips {
                    black_box(store.add(*ip));
                }
            });
        });
    }

    group.finish();
}

fn bench_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("ip_store_contains");

    for size in [100, 1000, 10000].iter() {
        let ips = generate_ips(*size);
        let store = IpStoreImpl::new();

        // Pre-populate store
        for ip in &ips {
            store.add(*ip);
        }

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                for ip in &ips {
                    black_box(store.contains(ip));
                }
            });
        });
    }

    group.finish();
}

fn bench_concurrent_insert(c: &mut Criterion) {
    use std::sync::Arc;
    use std::thread;

    c.bench_function("ip_store_concurrent_insert", |b| {
        b.iter(|| {
            let store = Arc::new(IpStoreImpl::new());
            let mut handles = vec![];

            for thread_id in 0..4 {
                let store_clone = Arc::clone(&store);
                let handle = thread::spawn(move || {
                    for i in 0..250 {
                        let ip = IpAddr::from_str(&format!("192.168.{}.{}", thread_id, i)).unwrap();
                        black_box(store_clone.add(ip));
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

criterion_group!(
    benches,
    bench_insert,
    bench_contains,
    bench_concurrent_insert
);
criterion_main!(benches);
