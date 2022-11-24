use criterion::{measurement::WallTime, BenchmarkGroup};

pub mod hex {
    pub use hex::{decode, encode};
}

pub mod rng {
    pub use rand::rngs::OsRng;
}

pub use lazy_static;

pub const DUMMY_BUF_SIZE: usize = 4096;
pub const DUMMY_BUF_DATA: [u8; DUMMY_BUF_SIZE] = [0; DUMMY_BUF_SIZE];

pub fn print_str(s: &str) {
    println!("{}", s);
}

pub fn run_bench(name: &str, group: &mut BenchmarkGroup<WallTime>, mut f: impl FnMut()) {
    group.bench_function(name, |b| b.iter(|| criterion::black_box(f())));
}
