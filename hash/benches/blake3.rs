use criterion::{criterion_group, criterion_main, Criterion};
use utils::{run_bench, DUMMY_BUF_DATA};

mod blake3_benches {
    use super::*;
    use blake3::Hasher;

    pub fn hash_256() -> impl Fn() {
        move || {
            let mut digest: [u8; 32] = [0; 32];
            let mut ctx = Hasher::new();
            ctx.update(&DUMMY_BUF_DATA);
            ctx.update(&DUMMY_BUF_DATA);
            ctx.update(&DUMMY_BUF_DATA);
            let _ = ctx.finalize_xof().fill(&mut digest);
        }
    }

    pub fn hash_512() -> impl Fn() {
        move || {
            let mut digest: [u8; 64] = [0; 64];
            let mut ctx = Hasher::new();
            ctx.update(&DUMMY_BUF_DATA);
            ctx.update(&DUMMY_BUF_DATA);
            ctx.update(&DUMMY_BUF_DATA);
            let _ = ctx.finalize_xof().fill(&mut digest);
        }
    }
}

fn blake3(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("blake3-256");
        run_bench("blake3", &mut group, blake3_benches::hash_256());
    }
    {
        let mut group = c.benchmark_group("blake3-512");
        run_bench("blake3", &mut group, blake3_benches::hash_512());
    }
}

criterion_group!(benches, blake3);
criterion_main!(benches);
