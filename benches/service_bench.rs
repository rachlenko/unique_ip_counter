// benches/service_bench.rs - Performance benchmarks for IpCounterService

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use std::sync::Arc;
use unique_ip_counter::models::LogEntry;
use unique_ip_counter::services::IpCounterService;
use unique_ip_counter::storage::{IpStoreImpl, MockMetricsStore};

fn generate_log_entries(count: usize) -> Vec<LogEntry> {
    (1..=count)
        .map(|i| {
            let octet1 = (i / (256 * 256)) % 256;
            let octet2 = (i / 256) % 256;
            let octet3 = i % 256;
            let ip_str = format!("10.{}.{}.{}", octet1, octet2, octet3);
            LogEntry::new(ip_str, None)
        })
        .collect()
}

fn bench_service_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("service_process_log_entry");

    for size in [1000, 5000, 10000].iter() {
        let log_entries = generate_log_entries(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let ip_store = Arc::new(IpStoreImpl::new());
                let metrics_store = Arc::new(MockMetricsStore::new());
                let service = IpCounterService::new(ip_store, metrics_store);

                for entry in &log_entries {
                    black_box(service.process_log_entry(entry).unwrap());
                }
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_service_processing);
criterion_main!(benches);
