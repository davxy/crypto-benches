use criterion::{measurement::WallTime, BenchmarkGroup};

pub fn print_str(s: &str) {
    println!("{}", s);
}

pub fn run_bench(name: &str, group: &mut BenchmarkGroup<WallTime>, mut f: impl FnMut()) {
    group.bench_function(name, |b| b.iter(|| criterion::black_box(f())));
}
